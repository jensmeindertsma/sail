mod shutdown;
mod socket;

use crate::application::{shutdown::exit::handle_socket_exit, socket::SocketError};
use shutdown::setup_shutdown_handler;
use socket::handle_socket;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    time::Duration,
};
use tokio::{join, time::timeout};
use tracing::{Instrument, info_span};

pub async fn run() -> Result<(), Failure> {
    tracing::info!("starting up");

    let mut shutdown_signal = setup_shutdown_handler().map_err(Failure::Signal)?;

    let mut socket_task =
        tokio::spawn(handle_socket(shutdown_signal.clone()).instrument(info_span!("socket")));

    tokio::select! {
        biased;

         _ = shutdown_signal.changed() => {
            tracing::info!("received shutdown signal");
            }

        output = &mut socket_task => handle_socket_exit(output)?,

    }

    let grace_period = Duration::from_secs(5);
    let socket_res = join!(timeout(grace_period, &mut socket_task)).0;

    if socket_res.is_err() {
        tracing::warn!("socket task did not shutdown within the grace period");
    }

    tracing::info!("shutdown complete");

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    Signal(io::Error),
    Socket(SocketError),
    Task(Task),
}

#[derive(Debug)]
pub enum Task {
    Socket,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signal(io_error) => write!(f, "failed to set up shutdown listener: {io_error}"),
            Self::Socket(socket_error) => write!(f, "socket handler: {socket_error}"),
            Self::Task(task) => write!(
                f,
                "{} task crashed",
                match task {
                    Task::Socket => "socket",
                }
            ),
        }
    }
}

impl Error for Failure {}
