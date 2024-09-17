use sail_core::socket::{SocketMessage, SocketReply, SocketRequest, SocketResponse};
use std::{
    io::{self, BufRead, BufReader, Lines, Write},
    os::unix::net::UnixStream,
    path::Path,
};

pub struct Socket {
    reader: Lines<BufReader<UnixStream>>,
    writer: UnixStream,
    next_id: u8,
}

impl Socket {
    pub fn connect(socket_path: impl AsRef<Path>) -> Result<Self, SocketConnectError> {
        let stream = UnixStream::connect(socket_path)?;

        Ok(Self {
            reader: BufReader::new(stream.try_clone()?).lines(),
            writer: stream,
            next_id: 1,
        })
    }

    pub fn send_request(&mut self, request: SocketRequest) -> Result<SocketResponse, SocketError> {
        let message = SocketMessage {
            id: self.next_id,
            request,
        };

        self.next_id += 1;

        self.writer
            .write_all(
                format!(
                    "{}\n",
                    serde_json::to_string(&message)
                        .map_err(|e| SocketError::FailedSerialization(e))?
                )
                .as_bytes(),
            )
            .map_err(|e| SocketError::WriteFailure(e))?;

        let reply: SocketReply = serde_json::from_str(
            &self
                .reader
                .next()
                .ok_or(SocketError::NoReply)?
                .map_err(|e| SocketError::ReadFailure(e))?,
        )
        .map_err(|e| SocketError::FailedDeserialization(e))?;

        if reply.regarding != message.id {
            return Err(SocketError::ReplyMismatch);
        }

        Ok(reply.response)
    }
}

pub struct SocketConnectError(pub io::Error);

impl From<io::Error> for SocketConnectError {
    fn from(value: io::Error) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum SocketError {
    FailedDeserialization(serde_json::Error),
    FailedSerialization(serde_json::Error),
    NoReply,
    ReadFailure(io::Error),
    ReplyMismatch,
    WriteFailure(io::Error),
}
