use sail_core::socket::{SocketMessage, SocketReply, SocketResponse};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf, SocketAddr},
        UnixStream,
    },
};
use tracing::error;

pub struct SocketConnection {
    reader: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
    pub address: SocketAddr,
}

impl SocketConnection {
    pub fn new(stream: UnixStream, address: SocketAddr) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader: BufReader::new(reader).lines(),
            writer,
            address,
        }
    }

    pub async fn next_message(&mut self) -> Option<SocketMessage> {
        match self.reader.next_line().await {
            Ok(maybe_line) => match serde_json::from_str(&maybe_line?) {
                Ok(message) => message,
                Err(error) => {
                    error!("failed to deserialize incoming message: {error}");
                    None
                }
            },
            Err(error) => {
                error!("failed to read from the socket: {error}");

                // If there is an issue with reading from the socket, we should
                // close the connection.
                self.reply(SocketReply {
                    regarding: u8::MAX,
                    response: SocketResponse::ConnectionClosed,
                })
                .await;

                None
            }
        }
    }

    pub async fn reply(&mut self, reply: SocketReply) {
        let serialized_reply = match serde_json::to_string(&reply) {
            Ok(string) => string,
            Err(error) => {
                error!("failed to serialize outgoing reply: {error}");
                return;
            }
        };

        if let Err(error) = self
            .writer
            .write_all(format!("{serialized_reply}\n").as_bytes())
            .await
        {
            error!("failed to write to the socket: {error}");
        };
    }
}
