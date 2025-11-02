mod shutdown;
mod socket;

use futures::future::join_all;
use shutdown::setup_shutdown_handler;
use socket::{SocketError, handle_socket};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};
use tokio::task::JoinError;
use tracing::{Instrument, info_span};

#[tracing::instrument(name = "daemon")]
pub async fn run() -> Result<(), Failure> {
    tracing::info!("starting up");

    let mut shutdown_signal = setup_shutdown_handler().map_err(Failure::Signal)?;

    let mut socket_task =
        tokio::spawn(handle_socket(shutdown_signal.clone()).instrument(info_span!("socket")));

    tokio::select! {
        biased;

         _ = shutdown_signal.changed() => {
            tracing::info!("shutdown signal received ");
            }

        output = &mut socket_task => {
            return output.map_err(Failure::Task)?.map_err(Failure::Socket)
        },
    }

    join_all(vec![socket_task]).await;

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    Signal(io::Error),
    Socket(SocketError),
    Task(JoinError),
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signal(io_error) => write!(f, "failed to set up shutdown listener: {io_error}"),
            Self::Socket(socket_error) => write!(f, "socket handler failed: {socket_error}"),
            Self::Task(join_error) => write!(f, "task crashed: {join_error}"),
        }
    }
}

impl Error for Failure {}
