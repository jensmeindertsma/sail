use std::io;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

pub async fn handle_connection(mut stream: UnixStream) -> Result<SocketRequest, HandlerError> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // Read a single line from the client
    let bytes_read = buf_reader.read_line(&mut line).await?;
    if bytes_read == 0 {
        return Err(HandlerError::EndOfFile);
    }

    let request = serde_json::from_str(line).map_err(|_| HandlerError::InvalidRequest)?;

    tracing::info!("received request: {request:?}");

    let response = match request {
        SocketRequest::Status => {
            SocketResponse::Status {
                uptime: HOW TO GET
            }
        }
    };

    writer.write_all(serde_json::to_string(response).unwrap().as_bytes()).await?;
    writer.flush().await?;

    Ok(request)
}

pub enum HandlerError {
    EndOfFile,
    Io(io::Error),
}

impl From<io::Error> for HandlerError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub enum SocketRequest {
    Status,
}
