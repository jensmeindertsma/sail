mod connection;

use crate::program::state::State;
use arc_swap::ArcSwap;
use connection::handle_connection;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    os::{fd::FromRawFd, unix::net::UnixListener as StdUnixListener},
    sync::Arc,
};
use tokio::{net::UnixListener, sync::watch::Receiver, task::JoinSet};
use tracing::{Instrument, info_span};

pub async fn handle_socket(
    mut signal: Receiver<()>,
    state: Arc<ArcSwap<State>>,
) -> Result<(), SocketError> {
    let listener = {
        let std_fd = unsafe { StdUnixListener::from_raw_fd(3) };
        std_fd
            .set_nonblocking(true)
            .map_err(SocketError::CreateListener)?;
        tracing::info!("creating listener");
        UnixListener::from_std(std_fd).map_err(SocketError::CreateListener)?
    };

    let mut handlers = JoinSet::new();
    let mut connection_id = 1;

    loop {
        tokio::select! {
            biased;

            _ = signal.changed() => {
                tracing::info!("received shutdown signal");
                return Ok(())
            }

            _ = handlers.join_next() => {}

            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                       handlers.spawn(handle_connection(stream, state.clone()).instrument(info_span!("connection", id=connection_id)));
                        connection_id += 1;
                    },
                    Err(_) => {
                        tracing::error!("failed to accept new connection");
                    },
                }
            }



        }
    }
}

#[derive(Debug)]
pub enum SocketError {
    Accept(io::Error),
    CreateListener(io::Error),
    Crash,
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept(io_error) => write!(f, "failed to accept socket connection: {io_error}"),
            Self::CreateListener(io_error) => {
                write!(f, "failed to create socket listener: {io_error}")
            }
            Self::Crash => write!(f, "we are so cooked"),
        }
    }
}

impl Error for SocketError {}
