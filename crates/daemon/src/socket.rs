use core::fmt::{self, Formatter};
use sail_core::{SocketRequest, SocketResponse};
use std::{
    convert::Infallible,
    env::{self, VarError},
    error::Error,
    io::{self},
    num::ParseIntError,
    os::fd::FromRawFd,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixListener,
};
use tower::Service;
use tracing::{error, info, info_span, Instrument};

pub struct Socket {
    listener: UnixListener,
}

impl Socket {
    pub fn attach() -> Result<Self, AttachmentError> {
        let var = env::var("LISTEN_FDS").map_err(AttachmentError::BadEnvironment)?;

        let fd_count: i32 = var
            .parse()
            .map_err(AttachmentError::InvalidFileDescriptor)?;

        if fd_count != 1 {
            return Err(AttachmentError::UnexpectedFileDescriptorCount(fd_count));
        }

        // SAFETY: this file descriptor comes from systemd
        // For more detail, see https://www.man7.org/linux/man-pages/man3/sd_listen_fds.3.html
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };

        std_listener
            .set_nonblocking(true)
            .map_err(AttachmentError::ConversionFailure)?;

        let listener =
            UnixListener::from_std(std_listener).map_err(AttachmentError::ConversionFailure)?;

        Ok(Self { listener })
    }

    pub async fn serve_connections<S>(&self, service: S)
    where
        S: Service<SocketRequest, Response = SocketResponse, Error = Infallible>,
        S: Clone,
        S: Send + 'static,
        S::Future: Send + 'static,
    {
        let mut count = 1;
        loop {
            let (stream, _) = match self.listener.accept().await {
                Ok(connection) => connection,
                Err(error) => {
                    error!("failed to accept new connection: {error}");
                    continue;
                }
            };

            let id = count;
            count += 1;

            info!("accepted new connection");

            let mut new_service = service.clone();
            tokio::spawn(
                async move {
                    let (reader, mut writer) = stream.into_split();
                    let mut reader = BufReader::new(reader).lines();

                    while let Ok(Some(line)) = reader.next_line().await {
                        let request: SocketRequest = match serde_json::from_str(&line) {
                            Ok(request) => request,
                            Err(error) => {
                                error!("failed to deserialize request: {error}");
                                continue;
                            }
                        };

                        info!("received request: {request:?}");

                        let Ok(response) = new_service.call(request).await;

                        info!("sending response: {response:?}");

                        if let Err(error) = writer
                            .write_all(
                                format!(
                                    "{}\n",
                                    match serde_json::to_string(&response) {
                                        Ok(string) => string,
                                        Err(error) => {
                                            error!("failed to serialize response: {error}");
                                            continue;
                                        }
                                    }
                                )
                                .as_bytes(),
                            )
                            .await
                        {
                            error!("failed to write response: {error}");
                            continue;
                        };
                    }
                }
                .instrument(info_span!("handler", id)),
            );
        }
    }
}

#[derive(Debug)]
pub enum AttachmentError {
    BadEnvironment(VarError),
    ConversionFailure(io::Error),
    InvalidFileDescriptor(ParseIntError),
    UnexpectedFileDescriptorCount(i32),
}

impl fmt::Display for AttachmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadEnvironment(error) => match error {
                VarError::NotPresent => write!(f, "required environment variable `LISTEN_FDS` pointing to the systemd socket is not present, please run this daemon under systemd!"),
                VarError::NotUnicode(str) => write!(f, "value `{str:?}` of environment variable `LISTEN_FDS` is not unicode")
            },
            Self::ConversionFailure(error) => write!(f, "failed to convert `std::os::unix::net::UnixListener` to asynchronous Tokio version: {error}"),
            Self::InvalidFileDescriptor(error) => write!(f, "failed to parse file descriptor number from `LISTEN_FDS` value: {error}"),
            Self::UnexpectedFileDescriptorCount(fd) => write!(f, "expected a file descriptor count of 1, as Sail only uses one socket, instead got `{fd}` as a value?"),
        }
    }
}

impl Error for AttachmentError {}
