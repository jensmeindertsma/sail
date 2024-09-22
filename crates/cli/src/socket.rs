use core::fmt::{self, Formatter};
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
    pub fn connect(socket_path: impl AsRef<Path>) -> Result<Self, ()> {
        let stream = UnixStream::connect(socket_path).map_err(|_| ())?;

        Ok(Self {
            reader: BufReader::new(stream.try_clone().map_err(|_| ())?).lines(),
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
                    serde_json::to_string(&message).map_err(SocketError::FailedSerialization)?
                )
                .as_bytes(),
            )
            .map_err(SocketError::WriteFailure)?;

        let reply: SocketReply = serde_json::from_str(
            &self
                .reader
                .next()
                .ok_or(SocketError::NoReply)?
                .map_err(SocketError::ReadFailure)?,
        )
        .map_err(SocketError::FailedDeserialization)?;

        if reply.regarding != message.id {
            return Err(SocketError::ReplyMismatch);
        }

        Ok(reply.response)
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

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedDeserialization(error) => {
                write!(f, "failed to deserialize reply: {error}")
            }
            Self::FailedSerialization(error) => {
                write!(f, "failed to serialize request: {error}")
            }
            Self::NoReply => write!(f, "received no reply from daemon"),
            Self::ReadFailure(io_error) => {
                write!(f, "failed to read from the socket: {io_error}")
            }
            Self::ReplyMismatch => write!(f, "incoming reply has ID mismatch"),
            Self::WriteFailure(io_error) => {
                write!(f, "failed to write to the socket: {io_error}")
            }
        }
    }
}
