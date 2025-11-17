use crate::program::state::State;
use arc_swap::ArcSwap;
use instruct::{SocketRequest, SocketResponse};
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::UnixStream,
};

pub async fn handle_connection(mut stream: UnixStream, state: Arc<ArcSwap<State>>) {
    let (reader, writer) = stream.split();

    let Ok(request) = parse_request(reader).await else {
        return;
    };

    let response = match request {
        SocketRequest::Status => SocketResponse::Status {
            count: state.load().count,
        },
        SocketRequest::Add => {
            let mut copy = state.load().as_ref().clone();

            copy.count += 1;

            copy.save_to_file().await;

            state.store(Arc::new(copy));

            SocketResponse::AddComplete
        }
    };

    let _ = reply(writer, response).await;
}

async fn parse_request(stream: impl AsyncRead + Unpin) -> Result<SocketRequest, ()> {
    let mut buffer = BufReader::new(stream);
    let mut line = String::new();

    match buffer.read_line(&mut line).await {
        Ok(0) => {
            tracing::error!("stream is empty");
            return Err(());
        }
        Err(_) => {
            tracing::error!("failed to read from stream")
        }

        _ => {}
    };

    serde_json::from_str(line.trim()).map_err(|_| tracing::error!("failed to deserialize request"))
}

async fn reply(mut stream: impl AsyncWrite + Unpin, response: SocketResponse) -> Result<(), ()> {
    let text = serde_json::to_string(&response)
        .map_err(|_| tracing::error!("failed to serialize response"))?;

    stream
        .write_all(text.as_bytes())
        .await
        .map_err(|_| tracing::error!("failed to write to stream"))?;

    stream
        .flush()
        .await
        .map_err(|_| tracing::error!("failed to flush stream"))
}
