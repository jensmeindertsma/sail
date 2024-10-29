mod server;
mod shutdown;
mod socket;

use server::Server;
use shutdown::setup_shutdown_listener;
use socket::Socket;
use std::process::ExitCode;
use tokio::time::{sleep, Duration};
use tracing::{error, info, info_span, Instrument, Level};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let (mut shutdown_signal, _) = setup_shutdown_listener();

    let socket = match Socket::attach() {
        Ok(socket) => socket,
        Err(error) => {
            error!("failed to attach to socket: {error}");
            return ExitCode::FAILURE;
        }
    };

    let socket_task = tokio::spawn(
        socket
            .serve_connections(SocketHandler::new(), shutdown_signal.clone())
            .instrument(info_span!("socket")),
    );

    let server = match Server::bind("127.0.0.1:4250").await {
        Ok(server) => server,
        Err(error) => {
            error!("failed to bind to `127.0.0.1:4250`: {error}");
            return ExitCode::FAILURE;
        }
    };

    let server_result = server
        .serve_connections(ServerHandler::new(), shutdown_signal, &socket_task)
        .instrument(info_span!("server"))
        .await;

    let remaining_connections = match server_result {
        Ok(watcher) => watcher,
        Err(error) => {
            error!("server failed");
            return ExitCode::FAILURE;
        }
    };

    tokio::select! {
        // Calling `.shutdown()` on the handle returned by the server after it stops its connection
        // loop, will gracefully terminate current connections and return a future that resolves
        // when this is finished.
        _ = remaining_connections.shutdown() => {
            info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            error!("timed out wait for all connections to close");
        }
    }

    if let Err(error) = socket_task.await {
        error!("failed to shutdown socket handler: {error}")
    };

    ExitCode::SUCCESS
}
