use sail_core::{SocketRequest, SocketResponse};
use std::{convert::Infallible, fmt::Display, io};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    net::{
        UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use tower::Service;

pub struct SocketConnector {
    reader: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
}

impl SocketConnector {
    pub fn new(stream: UnixStream) -> Self {
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader).lines();

        Self { reader, writer }
    }

    pub async fn serve_connection<S>(&mut self, mut service: S)
    where
        S: Service<SocketRequest, Response = SocketResponse, Error = Infallible>,
    {
        while let Ok(Some(request)) = self.receive().await {
            let response = service.call(request).await.expect("should be infallible");

            if let Err(error) = self.send(response).await {
                tracing::error!("error while serving socket connection: {error}");

                // Close the connection upon error.
                return;
            }
        }
    }

    async fn receive(&mut self) -> Result<Option<SocketRequest>, ConnectorError> {
        match self.reader.next_line().await {
            Ok(maybe_line) => Ok(match maybe_line {
                Some(line) => {
                    Some(serde_json::from_str(&line).map_err(ConnectorError::Serialization)?)
                }
                _ => None,
            }),
            Err(io_error) => Err(ConnectorError::Io(io_error)),
        }
    }

    async fn send(&mut self, response: SocketResponse) -> Result<(), ConnectorError> {
        self.writer
            .write_all(
                format!(
                    "{}\n",
                    serde_json::to_string(&response).map_err(ConnectorError::Serialization)?
                )
                .as_bytes(),
            )
            .await
            .map_err(ConnectorError::Io)?;

        Ok(())
    }
}

#[derive(Debug)]
enum ConnectorError {
    Io(io::Error),
    Serialization(serde_json::Error),
}

impl Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io_error) => write!(f, "failed to read/write from the socket: {io_error}"),
            Self::Serialization(serde_error) => {
                write!(f, "encountered (de) serialization error: {serde_error}")
            }
        }
    }
}

impl std::error::Error for ConnectorError {}
