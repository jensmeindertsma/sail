mod connection;

use crate::application::socket::connection::HandlerError;

use super::shutdown::ShutdownSignal;
use connection::handle_connection;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    os::{fd::FromRawFd, unix::net::UnixListener as StdUnixListener},
};
use tokio::{net::UnixListener, task::JoinSet};
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

    let mut connections = JoinSet::new();

    loop {
        tokio::select! {
            biased;

            _ = shutdown_signal.changed() => {
                return Ok(())
            }

            Some(result) = connections.join_next() => {
                match result {
                    Ok(Err(HandlerError::EndOfFile)) => {},
                    Ok(Handler)
                    _ => {}
                };
            }

            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                        tracing::info!("handling new connection");
                       connections.spawn(handle_connection(stream).instrument(info_span!("handler")));
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
