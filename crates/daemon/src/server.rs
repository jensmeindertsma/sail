use core::fmt::{self, Formatter};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto::Builder, graceful::GracefulShutdown},
    service::TowerToHyperService,
};
use std::error::Error;
use tokio::net::TcpStream;

pub struct Server {}

impl Server {
    pub async fn new() -> Self {}

    pub async fn accept(&mut self) -> Result<TcpStream, ()> {}
}

#[derive(Debug)]
pub enum ServerError {}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo!")
    }
}

impl Error for ServerError {}
