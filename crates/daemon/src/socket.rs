mod connection;

use connection::SocketConnection;
use core::fmt::{self, Formatter};
use std::{
    env::{self, VarError},
    num::ParseIntError,
    os::fd::FromRawFd,
};
use tokio::{io, net::UnixListener};
use tracing::{error, info};

pub struct Socket {
    listener: UnixListener,
}

impl Socket {
    pub fn attach() -> Result<Self, SocketError> {
        let var = env::var("LISTEN_FDS").map_err(|e| SocketError::BadEnvironment(e))?;

        let fd_count: i32 = var
            .parse()
            .map_err(|e| SocketError::InvalidFileDescriptor(e))?;

        if fd_count != 1 {
            return Err(SocketError::UnexpectedFileDescriptorCount(fd_count));
        }

        // SAFETY: this file descriptor comes from systemd
        // For more detail, see https://www.man7.org/linux/man-pages/man3/sd_listen_fds.3.html
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };

        std_listener
            .set_nonblocking(true)
            .map_err(|e| SocketError::ConversionFailure(e))?;

        Ok(Self {
            listener: UnixListener::from_std(std_listener)
                .map_err(|e| SocketError::ConversionFailure(e))?,
        })
    }

    pub async fn accept(&mut self) -> Option<SocketConnection> {
        match self.listener.accept().await {
            Ok((stream, socket_address)) => {
                info!("accepting new socket connection from {socket_address:?}");
                return Some(SocketConnection::new(stream));
            }
            Err(error) => {
                error!("socket listener failed to accept new connection: {error:?}");
                return None;
            }
        };
    }
}

#[derive(Debug)]
pub enum SocketError {
    BadEnvironment(VarError),
    ConversionFailure(io::Error),
    InvalidFileDescriptor(ParseIntError),
    UnexpectedFileDescriptorCount(i32),
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadEnvironment(error) => match error {
                VarError::NotPresent => write!(f, "required environment variable `LISTEN_FDS` pointing to the systemd socket is not present, please run this daemon under systemd!"),
                VarError::NotUnicode(str) => write!(f, "value `{str:?}` of environment variable `LISTEN_FDS` is not unicode")
            },
            Self::ConversionFailure(error) => write!(f, "failed to convert `std::os::unix::net::UnixListener` to asynchronous Tokio version: {error:?}"),
            Self::InvalidFileDescriptor(error) => write!(f, "failed to parse file descriptor number from `LISTEN_FDS` value: {error:?}"),
            Self::UnexpectedFileDescriptorCount(fd) => write!(f, "expected a file descriptor count of 1, as Sail only uses one socket, instead got `{fd}` as a value?"),
        }
    }
}
