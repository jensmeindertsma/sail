use crate::configuration::Configuration;
use core::fmt::{self, Formatter};
use std::{
    error::Error,
    io,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn listen(configuration: Arc<Configuration>) -> Result<Self, ServerListenError> {
        let port = configuration.get().server_port;
        let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), port))
            .await
            .map_err(|error| ServerListenError::BindFailure { error, port })?;

        Ok(Self { listener })
    }

    pub async fn accept(&mut self) -> Result<ServerConnection, io::Error> {
        self.listener
            .accept()
            .await
            .map(|(stream, address)| ServerConnection { stream, address })
    }
}

#[derive(Debug)]
pub enum ServerListenError {
    BindFailure { error: io::Error, port: u16 },
}

impl fmt::Display for ServerListenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BindFailure { error, port } => {
                write!(f, "server failed to bind to port {port}: {error}")
            }
        }
    }
}

impl Error for ServerListenError {}

pub struct ServerConnection {
    pub stream: TcpStream,
    pub address: SocketAddr,
}
