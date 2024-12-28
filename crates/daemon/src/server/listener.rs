use std::io;
use tokio::net::{TcpListener, TcpStream};

// This abstraction is fairly useless but I'm trying to keep the abstraction
// similar between both socket and server. Each has a listener, a connection
// handler and a service.

pub struct ServerListener {
    listener: TcpListener,
}

impl ServerListener {
    pub async fn bind(port: u16) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(("127.0.0.1", port)).await?;

        Ok(Self { listener })
    }

    pub async fn accept(&mut self) -> Result<TcpStream, io::Error> {
        self.listener.accept().await.map(|(stream, _)| stream)
    }
}
