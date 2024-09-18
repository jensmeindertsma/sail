mod configuration;
mod handlers;
mod server;
mod shutdown;
mod socket;
mod startup;

use configuration::Configuration;
use shutdown::setup_shutdown_listener;
use startup::{start_server_handler, start_socket_handler};
use std::{process::ExitCode, sync::Arc};
use tokio::time::{sleep, Duration};
use tracing::{error, info, info_span, Instrument, Level};

#[tokio::main]
async fn main() -> ExitCode {
    let mut failure = false;

    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    info!("starting up...");

    let (shutdown_signal, request_shutdown) = setup_shutdown_listener();

    let configuration = Arc::new(Configuration::load());

    info!("loaded configuration: {:?}", configuration.get());

    let socket_task_handle = tokio::spawn(
        start_socket_handler(
            configuration.clone(),
            shutdown_signal.clone(),
            request_shutdown,
        )
        .instrument(info_span!("socket")),
    );

    match start_server_handler(configuration, shutdown_signal)
        .instrument(info_span!("server"))
        .await
    {
        Ok(handle) => {
            // We only get here after the server connection loop inside the
            // `start_server_handler` function has been broken. This happens
            // because shutdown signal was received.

            // Now, we allow the server to close its connections
            // gracefully.

            info!("server has stopped, waiting for all current connections to be closed");

            tokio::select! {
                _ = handle.shutdown() => {
                    info!("all connections gracefully closed");
                },
                _ = sleep(Duration::from_secs(10)) => {
                    error!("timed out wait for all connections to close");
                }
            }
        }
        Err(error) => {
            // Server stopped due to critical problem.
            failure = true;

            error!("server experienced critical failure: {error:?}");
        }
    };

    if let Err(error) = socket_task_handle.await {
        error!("failed to complete socket handler task: {error:?}");
        failure = true
    } else {
        info!("successfully stopped the socket handler")
    }

    info!("shutting down...");

    if failure {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
