mod handlers;
mod server;
mod shutdown;
mod socket;

use handlers::{ServerHandler, SocketHandler};
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

    let (shutdown_signal, request_shutdown) = setup_shutdown_listener();

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

    // We need to monitor the socket task: if it stops unexpectedly, we should
    // shutdown everything else!
    let mut watcher_shutdown_signal = shutdown_signal.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = watcher_shutdown_signal.changed() => return,
            _ = socket_task => {
                error!("socket task stopped unexpectedly, shutting down!");
                request_shutdown();
            }
        }
    });

    let server = match Server::bind("127.0.0.1:4250").await {
        Ok(server) => server,
        Err(error) => {
            error!("failed to bind to `127.0.0.1:4250`: {error}");
            return ExitCode::FAILURE;
        }
    };

    let remaining_connections = server
        .serve_connections(ServerHandler::new(), shutdown_signal)
        .instrument(info_span!("server"))
        .await;

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
