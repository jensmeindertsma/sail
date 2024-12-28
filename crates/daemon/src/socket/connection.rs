use sail_core::{SocketRequest, SocketResponse};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    net::{
        UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct SocketConnection {
    reader: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
}

impl SocketConnection {
    pub fn new(stream: UnixStream) -> Self {
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader).lines();

        Self { reader, writer }
    }

    pub async fn receive(&mut self) -> SocketRequest {
        serde_json::from_str(&self.reader.next_line().await.unwrap().unwrap()).unwrap()
    }

    pub async fn send(&mut self, response: SocketResponse) {
        self.writer
            .write_all(serde_json::to_string(&response).unwrap().as_bytes())
            .await
            .unwrap();
    }
}
