use std::{
    env::{self, VarError},
    fmt::{self, Display, Formatter},
    io,
    num::ParseIntError,
    os::fd::FromRawFd,
};
use tokio::net::{UnixListener, UnixStream};

pub struct SocketListener {
    listener: UnixListener,
}

impl SocketListener {
    pub fn attach() -> Result<Self, ListenerError> {
        let var = env::var("LISTEN_FDS").map_err(ListenerError::MissingVariable)?;

        let fd_count: i32 = var.parse().map_err(ListenerError::InvalidVariable)?;

        if fd_count != 1 {
            return Err(ListenerError::UnexpectedValue(fd_count));
        }

        // SAFETY: this file descriptor comes from systemd
        // For more detail, see https://www.man7.org/linux/man-pages/man3/sd_listen_fds.3.html
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };

        std_listener
            .set_nonblocking(true)
            .map_err(ListenerError::UnblockFailure)?;

        let listener =
            UnixListener::from_std(std_listener).map_err(ListenerError::ConversionFailure)?;

        Ok(Self { listener })
    }

    pub async fn accept(&self) -> Result<UnixStream, io::Error> {
        self.listener.accept().await.map(|(stream, _)| stream)
    }
}

#[derive(Debug)]
pub enum ListenerError {
    ConversionFailure(io::Error),
    InvalidVariable(ParseIntError),
    MissingVariable(VarError),
    UnblockFailure(io::Error),
    UnexpectedValue(i32),
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionFailure(io_error) => {
                write!(f, "failed to convert listener: {io_error}")
            }
            Self::InvalidVariable(parse_error) => {
                write!(f, "failed to parse variable: {parse_error}")
            }
            Self::MissingVariable(var_error) => {
                write!(f, "missing environment variable `LISTEN_FDS`: {var_error}")
            }
            Self::UnblockFailure(io_error) => {
                write!(f, "failed to set listener to non-blocking: {io_error}")
            }
            Self::UnexpectedValue(value) => {
                write!(f, "unexpected value `{value}` for variable `LISTEN_FDS`")
            }
        }
    }
}

impl std::error::Error for ListenerError {}
