use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_connection(mut stream: TcpStream) {
    tracing::info!("replying with greeting");

    let message = b"Hello, server!\n";

    stream.write_all(message).await.unwrap();
}
