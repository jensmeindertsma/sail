mod shutdown;
mod socket;

use shutdown::create_shutdown_listener;
use socket::handle_socket;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::{
    select,
    time::{Duration, sleep},
};

pub async fn run() -> Result<(), ApplicationError> {
    tracing::info!("starting up...");

    let mut shutdown_signal = create_shutdown_listener();

    let mut socket_task = tokio::spawn(handle_socket(shutdown_signal.clone()));
    // let server_task = tokio::spawn(handle_server(shutdown_signal.clone()));

    select! {
        biased;

        _ = shutdown_signal.received() => {
            tracing::info!("shutdown signal received");
        }


        _ = &mut socket_task => {
            tracing::error!("socket task crashed unexpectedly");
        }

        // _ = server_task => {
        //     tracing::error!("server task crashed unexpectedly");
        // }
    };

    // TODO: Handle shutdown
    select! {
        biased;

        _ = sleep(Duration::from_secs(2)) =>  {
            tracing::warn!("timeout reached while gracefully shutting down");
        }

        _ = socket_task => {
        tracing::info!("socket task completed");
        }

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
