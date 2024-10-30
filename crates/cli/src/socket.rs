use core::fmt::{self, Formatter};
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Lines, Write},
    os::unix::net::UnixStream,
    path::Path,
};

use sail_core::{ConfigureError, SocketRequest, SocketResponse, Status};

pub struct Socket {
    reader: Lines<BufReader<UnixStream>>,
    writer: UnixStream,
}

impl Socket {
    pub fn connect(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let stream = UnixStream::connect(path)?;

        Ok(Self {
            reader: BufReader::new(stream.try_clone()?).lines(),
            writer: stream,
        })
    }

    fn send(&mut self, request: SocketRequest) -> Result<(), SocketError> {
        writeln!(
            self.writer,
            "{}",
            serde_json::to_string(&request).map_err(SocketError::Serialization)?
        )
        .map_err(SocketError::Io)?;

        Ok(())
    }

    fn receive(&mut self) -> Result<SocketResponse, SocketError> {
        let response = self
            .reader
            .next()
            .ok_or(SocketError::Ignored)?
            .map_err(SocketError::Io)?;

        let response = serde_json::from_str(&response).map_err(SocketError::Deserialization)?;

        Ok(response)
    }
}

#[derive(Debug)]
pub enum SocketError {
    Deserialization(serde_json::Error),
    Ignored,
    Io(io::Error),
    Serialization(serde_json::Error),
    UnexpectedResponse,
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deserialization(error) => write!(f, "failed to deserialize object: {error}"),
            Self::Ignored => write!(f, "server failed to respond to request"),
            Self::Io(error) => write!(f, "IO failure: {error}"),
            Self::Serialization(error) => write!(f, "failed to serialize object: {error}"),
            Self::UnexpectedResponse => write!(f, "received unexpected response from the server"),
        }
    }
}

impl Error for SocketError {}

impl Socket {
    pub fn request_configure(
        &mut self,
        setting: String,
        value: String,
    ) -> Result<Result<(), ConfigureError>, SocketError> {
        self.send(SocketRequest::Configure { setting, value })?;
        let response = self.receive()?;

        if let SocketResponse::Configure(result) = response {
            Ok(result)
        } else {
            Err(SocketError::UnexpectedResponse)
        }
    }

    pub fn request_status(&mut self) -> Result<Status, SocketError> {
        self.send(SocketRequest::Status)?;
        let response = self.receive()?;

        if let SocketResponse::Status(status) = response {
            Ok(status)
        } else {
            Err(SocketError::UnexpectedResponse)
        }
    }
}
