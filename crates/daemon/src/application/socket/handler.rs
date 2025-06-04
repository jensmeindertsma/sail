use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

pub async fn handle_connection(stream: UnixStream) {
    tracing::info!("handling new connection");

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader).lines();

    while let Some(message) = reader.next_line().await.unwrap() {
        tracing::debug!("read new message '{message}'");

        let reply = format!("received message '{message}'");

        tracing::debug!("replying with `{reply}`");

        writer
            .write_all(format!("{reply}\n").as_bytes())
            .await
            .unwrap();
    }

    tracing::info!("finished handling connection")
}
