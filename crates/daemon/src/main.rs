mod server;
mod shutdown;
mod socket;

use hyper_util::server::graceful::GracefulShutdown;
use server::Server;
use shutdown::setup_shutdown_listener;
use socket::Socket;
use tokio::time::{sleep, Duration};
use tracing::{error, info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let (shutdown_signal, request_shutdown) = setup_shutdown_listener();

    let socket_task = tokio::spawn(async move {
        let socket = Socket::attach()?;

        loop {
            tokio::select! {
                _ = shutdown_signal.changed() => {
                    break
                },
                connection = socket.accept() => {}
            }
        }

        Ok(())
    });

    let server = Server::new().await;
    let mut connections = GracefulShutdown::new();

    loop {
        tokio::select! {
            _ = shutdown_signal.changed() => {
                break
            },
            error = socket_task => {
                break
            }
            connection = server.accept() =>  {
                info!("new server connection!");

                // TODO register connection with gracefulshutdown
            }
        }
    }

    tokio::select! {
        // Calling `.shutdown()` on the handle returned by the server after it stops its connection
        // loop, will gracefully terminate current connections and return a future that resolves
        // when this is finished.
        _ = connections.shutdown() => {
            info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            error!("timed out wait for all connections to close");
        }
    }

    if let Err(error) = socket_task.await {
        error!("failed to shutdown socket handler: {error}")
    };
}
