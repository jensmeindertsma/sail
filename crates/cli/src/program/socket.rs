use sail_core::socket::{SocketRequest, SocketResponse};
use serde_json::Error as SerdeError;
use std::{
    error::Error,
    fmt::{self, Formatter},
    io::{self, BufRead, BufReader, ErrorKind, Lines, Write},
    os::unix::net::UnixStream,
    path::Path,
};

pub struct Socket {
    reader: Lines<BufReader<UnixStream>>,
    writer: UnixStream,
}

impl Socket {
    pub fn connect(path: impl AsRef<Path>) -> Result<Self, SocketError> {
        let stream = UnixStream::connect(path).map_err(|io_error| match io_error.kind() {
            ErrorKind::PermissionDenied => SocketError::PermissionDenied,
            _ => SocketError::Connect(io_error),
        })?;

        Ok(Self {
            reader: BufReader::new(stream.try_clone().map_err(SocketError::Connect)?).lines(),
            writer: stream,
        })
    }

    pub fn request(&mut self, request: SocketRequest) -> Result<SocketResponse, SocketError> {
        self.send(request)?;
        self.receive()
    }

    fn send(&mut self, request: SocketRequest) -> Result<(), SocketError> {
        writeln!(
            self.writer,
            "{}",
            serde_json::to_string(&request).map_err(SocketError::Serialization)?
        )
        .map_err(SocketError::Send)
    }

    fn receive(&mut self) -> Result<SocketResponse, SocketError> {
        let response = self
            .reader
            .next()
            .ok_or(SocketError::NoResponse)?
            .map_err(SocketError::Read)?;

        serde_json::from_str(&response).map_err(SocketError::Deserialization)
    }
}

#[derive(Debug)]
pub enum SocketError {
    Connect(io::Error),
    Deserialization(SerdeError),
    NoResponse,
    PermissionDenied,
    Read(io::Error),
    Serialization(SerdeError),
    Send(io::Error),
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connect(io_error) => write!(f, "failed to connect to socket: {io_error}"),
            Self::Deserialization(serde_error) => {
                write!(f, "failed to deserialize incoming message: {serde_error}")
            }
            Self::NoResponse => write!(f, "server did not reply to the message"),
            Self::PermissionDenied => {
                write!(f, "permission denied: cannot communicate with daemon")
            }
            Self::Read(io_error) => write!(f, "failed to read incoming message: {io_error}"),
            Self::Serialization(serde_error) => {
                write!(f, "failed to serialize outgoing message: {serde_error}")
            }
            Self::Send(io_error) => write!(f, "failed to send outgoing message: {io_error}"),
        }
    }
}

impl Error for SocketError {}
