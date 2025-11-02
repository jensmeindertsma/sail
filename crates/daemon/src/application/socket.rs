mod connection;

use super::shutdown::ShutdownSignal;
use connection::handle_connection;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    os::{fd::FromRawFd, unix::net::UnixListener as StdUnixListener},
};
use tokio::net::UnixListener;
use tracing::{Instrument, info_span};

pub async fn handle_socket(mut shutdown_signal: ShutdownSignal) -> Result<(), SocketError> {
    let listener = {
        let std_fd = unsafe { StdUnixListener::from_raw_fd(3) };
        std_fd
            .set_nonblocking(true)
            .map_err(SocketError::CreateListener)?;
        tracing::info!("creating listener");
        UnixListener::from_std(std_fd).map_err(SocketError::CreateListener)?
    };

    loop {
        tokio::select! {
            biased;

            _ = shutdown_signal.changed() => {
                return Ok(())
            }

            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                        tokio::spawn(handle_connection(stream).instrument(info_span!("handler")));
                    },
                    Err(error) => {
                        return Err(SocketError::Accept(error))
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
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept(io_error) => write!(f, "failed to accept socket connection: {io_error}"),
            Self::CreateListener(io_error) => {
                write!(f, "failed to create socket listener: {io_error}")
            }
        }
    }
}

impl Error for SocketError {}
