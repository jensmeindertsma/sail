use crate::configuration::Configuration;
use handler::{HandlerError, SocketHandler};
use sail_core::socket::{SocketRequest, SocketResponse};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    io,
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    net::{
        UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use tower::Service;

mod handler;

pub struct SocketConnection {
    configuration: Arc<Configuration>,
    reader: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
}

impl SocketConnection {
    pub fn new(configuration: Arc<Configuration>, stream: UnixStream) -> Self {
        let (reader, writer) = stream.into_split();

        Self {
            configuration,
            reader: BufReader::new(reader).lines(),
            writer,
        }
    }

    async fn read(&mut self) -> Result<Option<SocketRequest>, ReadError> {
        let Some(message) = self.reader.next_line().await.map_err(ReadError::Stream)? else {
            return Ok(None);
        };

        serde_json::from_str(&message)
            .map(Some)
            .map_err(ReadError::Deserialization)
    }

    async fn send(&mut self, response: SocketResponse) -> Result<(), SendError> {
        self.writer
            .write_all(
                format!(
                    "{}\n",
                    serde_json::to_string(&response).map_err(SendError::Serialization)?
                )
                .as_bytes(),
            )
            .await
            .map_err(SendError::Stream)
    }

    pub async fn serve(&mut self) -> Result<(), ConnectionError> {
        loop {
            let request: SocketRequest = match self.read().await.map_err(ConnectionError::Read)? {
                Some(request) => request,
                // `None` indicates the connection has no more messages available to read, so
                // we finish serving the connection.
                None => return Ok(()),
            };

            let mut handler = SocketHandler::new(self.configuration.clone());

            let response = handler
                .call(request)
                .await
                .map_err(ConnectionError::Handler)?;

            self.send(response).await.map_err(ConnectionError::Send)?
        }
    }
}

#[derive(Debug)]
pub enum ConnectionError {
    Handler(HandlerError),
    Read(ReadError),
    Send(SendError),
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handler(error) => write!(f, "{error}"),
            Self::Read(error) => write!(f, "{error}"),
            Self::Send(error) => write!(f, "{error}"),
        }
    }
}

impl Error for ConnectionError {}

#[derive(Debug)]
pub enum ReadError {
    Deserialization(serde_json::Error),
    Stream(io::Error),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deserialization(serde_error) => {
                write!(f, "failed to deserialize incoming message: {serde_error}")
            }
            Self::Stream(io_error) => write!(f, "failure while reading stream: {io_error}"),
        }
    }
}

impl Error for ReadError {}

#[derive(Debug)]
pub enum SendError {
    Serialization(serde_json::Error),
    Stream(io::Error),
}

impl Error for SendError {}

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(serde_error) => {
                write!(f, "failed to serialize outgoing message: {serde_error}")
            }
            Self::Stream(io_error) => write!(f, "failure while writing to stream: {io_error}"),
        }
    }
}
