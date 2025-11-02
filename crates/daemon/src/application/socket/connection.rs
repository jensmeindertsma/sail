use std::io;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

pub async fn handle_connection(mut stream: UnixStream) -> io::Result<()> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // Read a single line from the client
    let bytes_read = buf_reader.read_line(&mut line).await?;
    if bytes_read == 0 {
        // EOF: client closed the connection
        return Ok(());
    }

    // Trim newline characters
    let message = line.trim_end();

    tracing::info!("received message: {}", message);

    // Reply back
    let response = format!("received message: of {} length\n", message.len());
    writer.write_all(response.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
