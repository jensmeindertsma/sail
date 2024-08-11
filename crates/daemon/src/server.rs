use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Self, ServerError> {
        let listener = TcpListener::bind("127.0.0.1:4250")
            .await
            .map_err(|_e| ServerError::Bind)?;

        Ok(Self { listener })
    }

    pub async fn accept(&self) -> Result<ServerConnection, ServerError> {
        let (stream, address) = self
            .listener
            .accept()
            .await
            .map_err(|_e| ServerError::Accept)?;

        Ok(ServerConnection { stream, address })
    }
}

pub struct ServerConnection {
    pub stream: TcpStream,
    pub address: SocketAddr,
}

#[derive(Debug)]
pub enum ServerError {
    Accept,
    Bind,
}
