mod configuration;
mod server;
mod shutdown;
mod socket;

use configuration::Configuration;
use sail_core::SocketResponse;
use server::Server;
use shutdown::setup_shutdown_listener;
use socket::Socket;
use std::{process::ExitCode, sync::Arc, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> ExitCode {
    let mut shutdown_listener = setup_shutdown_listener();

    let configuration = Arc::new(Configuration::load());

    let socket_task = tokio::spawn(async move {
        let socket = Socket::attach();

        loop {
            let mut connection = socket.accept().await;

            tokio::spawn(async move {
                connection.receive().await;
                connection.send(SocketResponse::Welcome).await;
            });
        }
    });

    let server_task = tokio::spawn(async move {
        let server = Server::bind();

        loop {
            let mut connection = server.accept().await;

            // TODO spawn
        }
    });

    let mut socket_crashed = false;

    tokio::select! {
        biased;

        _ = socket_task => {
            tracing::error!("socket handler terminated unexpectedly!");
            socket_crashed = true
        }

        // When we receive the shutdown signal we break from the `select!`
        // and stop polling the futures.
        _ = shutdown_listener.changed() => {

        }

        // Only if we have no shutdown signal AND the socket handler is still active
        // do we process requests.
        _ = server_task => {}
    };

    // As soon as we leave the main `select!` phase we should attempt to
    // gracefully terminate existing connections.
    tokio::select! {
        _ = server_connections.shutdown() => {
            tracing::info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            tracing::error!("timed out wait for all connections to close");
        }
    };

    if socket_crashed {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
