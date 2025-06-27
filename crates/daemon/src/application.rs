mod shutdown;
mod socket;
mod uptime;

use shutdown::ShutdownSignal;
use socket::handle_socket;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::{
    select,
    time::{Duration, sleep},
};

use crate::application::uptime::log_uptime;

pub async fn run() -> Result<(), ApplicationError> {
    tracing::info!("starting up...");

    let mut shutdown_signal = ShutdownSignal::new();

    let uptime_task = tokio::spawn(log_uptime(shutdown_signal.clone()));

    let mut socket_task = tokio::spawn(handle_socket(shutdown_signal.clone()));
    // let server_task = tokio::spawn(handle_server(shutdown_signal.clone()));

    select! {
        biased;

        _ = shutdown_signal.receive() => {}

        _ = &mut socket_task => {
            tracing::error!("socket task crashed unexpectedly");
            shutdown_signal.broadcast();
        }

        // _ = server_task => {
        //     tracing::error!("server task crashed unexpectedly");
        // }

         _ = uptime_task => {
            tracing::info!("uptime task crashed unexpectedly");
            shutdown_signal.broadcast();
         }
    };

    // At least one task has stopped, potentially by crashing.
    // - If a task crashed, we want to signal shutdown to all other tasks
    //   (this is done with `shutdown_signal.broadcast()` above).
    // - If a shutdown signal was received before any tasks crashed, we
    //   want to wait for other tasks to shut down gracefully or a timeout
    //   to expire.

    tracing::info!("waiting for tasks to shut down gracefully");

    select! {
        biased;

        _ = sleep(Duration::from_secs(2)) =>  {
            tracing::warn!("timeout reached shutting down");
        }

        _ = socket_task => {}

        // _ = &mut server_task => {
        //     tracing::info!("server task shut down gracefully")
        // }
    };

    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "application error")
    }
}

impl Error for ApplicationError {}
