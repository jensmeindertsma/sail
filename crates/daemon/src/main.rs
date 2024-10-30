mod handlers;
mod server;
mod shutdown;
mod socket;

use handlers::{ServerHandler, SocketHandler};
use hyper_util::server::graceful::GracefulShutdown;
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

    let mut shutdown_signal = setup_shutdown_listener();

    let socket = match Socket::attach() {
        Ok(socket) => socket,
        Err(error) => {
            error!("failed to attach to socket: {error}");
            return ExitCode::FAILURE;
        }
    };

    info!("attached to the socket");

    let socket_task = tokio::spawn(
        async move {
            socket.serve_connections(SocketHandler::new()).await;
        }
        .instrument(info_span!("socket")),
    );

    let server = match Server::bind("127.0.0.1:4250").await {
        Ok(server) => server,
        Err(error) => {
            error!("failed to bind to `127.0.0.1:4250`: {error}");
            return ExitCode::FAILURE;
        }
    };

    info!("listening on `127.0.0.1:4250`");

    let watcher = GracefulShutdown::new();
    let mut failure = false;

    tokio::select! {
        biased;

        _ = socket_task => {
            error!("socket task stopped unexpectedly, shutting down!");
            failure = true
        }

        _ = shutdown_signal.changed() => {}

        _ = server
            .serve_connections(
                ServerHandler::new(),
                &watcher
            )
            .instrument(info_span!("server")) => {},
    };

    tokio::select! {
        // Calling `.shutdown()` on the handle returned by the server after it stops its connection
        // loop, will gracefully terminate current connections and return a future that resolves
        // when this is finished.
        _ = watcher.shutdown() => {
            info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            error!("timed out wait for all connections to close");
        }
    };

    if failure {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
