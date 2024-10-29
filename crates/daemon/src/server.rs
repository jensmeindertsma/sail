use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};
use std::convert::Infallible;
use tokio::{
    io,
    net::{TcpListener, ToSocketAddrs},
};
use tower::Service;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind(address: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(address).await?;

        Ok(Self { listener })
    }

    pub async fn serve_connections(&self, service: impl Service<Request<Incoming>>) {}
}

async fn hello_world(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
