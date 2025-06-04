mod shutdown;
mod socket;

use shutdown::create_shutdown_listener;
use socket::handle_socket;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::select;

pub async fn run() -> Result<(), ApplicationError> {
    tracing::info!("starting up...");

    let mut shutdown_signal = create_shutdown_listener();

    let socket_task = tokio::spawn(handle_socket(shutdown_signal.clone()));

    select! {
        biased;

        _ = shutdown_signal.received() => {
            tracing::info!("shutdown signal received")
        }

        _ = socket_task => {
            tracing::info!("socket task terminated")
        }
    }

    // Handle shutdown

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
