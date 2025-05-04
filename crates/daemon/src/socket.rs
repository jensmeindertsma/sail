use std::{
    env::{self, VarError},
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    num::ParseIntError,
    os::fd::FromRawFd,
};
use tokio::net::{UnixListener, UnixStream, unix::SocketAddr};

pub struct Socket {
    listener: UnixListener,
}

impl Socket {
    pub async fn attach() -> Result<Self, AttachmentError> {
        let fd_count: u8 = env::var("LISTEN_FDS")
            .map_err(|var_error| {
                AttachmentError::FileDescriptor(FileDescriptorError::Missing(var_error))
            })?
            .parse()
            .map_err(|parse_error| {
                AttachmentError::FileDescriptor(FileDescriptorError::Parsing(parse_error))
            })?;

        if fd_count != 1 {
            return Err(AttachmentError::FileDescriptor(
                FileDescriptorError::Unexpected(fd_count),
            ));
        }

        // SAFETY: this file descriptor comes from systemd
        // For more detail, see https://www.man7.org/linux/man-pages/man3/sd_listen_fds.3.html
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };

        std_listener
            .set_nonblocking(true)
            .map_err(AttachmentError::Conversion)?;

        let listener = UnixListener::from_std(std_listener).map_err(AttachmentError::Conversion)?;

        Ok(Self { listener })
    }

    pub async fn accept(&self) -> Result<(UnixStream, SocketAddr), io::Error> {
        self.listener.accept().await
    }
}

#[derive(Debug)]
pub enum AttachmentError {
    Conversion(io::Error),
    FileDescriptor(FileDescriptorError),
}

#[derive(Debug)]
pub enum FileDescriptorError {
    Parsing(ParseIntError),
    Missing(VarError),
    Unexpected(u8),
}

impl Display for AttachmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conversion(io_error) => write!(f, "failed to convert listener type: {io_error}"),
            Self::FileDescriptor(fd_error) => match fd_error {
                FileDescriptorError::Missing(_var_error) => {
                    write!(f, "missing required environment variable `LISTEN_FDS`")
                }
                FileDescriptorError::Parsing(_parse_error) => {
                    write!(f, "failed to parse socket file descriptor")
                }
                FileDescriptorError::Unexpected(value) => write!(
                    f,
                    "unexpected value `{value}` for socket file descriptor (expected `1`)"
                ),
            },
        }
    }
}

impl Error for AttachmentError {}
