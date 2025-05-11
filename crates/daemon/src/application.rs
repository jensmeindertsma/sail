mod shutdown;
mod socket;

use core::fmt::{self, Formatter};
use shutdown::handle_shutdown;
use socket::handle_socket;
use std::error::Error;
use tracing::instrument;

#[instrument(name = "application")]
pub async fn start() -> Result<(), ApplicationError> {
    let shutdown_signal = handle_shutdown();

    let configuration = Configuration::load().expect("loading configuration shouldn't fail");

    let socket_task = handle_socket(shutdown_signal);
    let registry_task = handle_registry(shutdown_signal);
    let web_task = handle_web(shutdown_signal);

    tokio::select! {
        result = socket_handle => {
            if let Err(error) = result {
                tracing::error!("socket handler crashed: {error:?}")
            }
        },

    };

    Ok(())

    // TODO: when the shutdown signal is received we should stop accepting new connections
}

#[derive(Debug)]
pub enum ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "todo!")
    }
}

impl Error for ApplicationError {}
