use crate::configuration::Configuration;
use http_body_util::Full;
use hyper::{
    body::{Body as HyperBody, Bytes, Frame, Incoming},
    Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{
    future::Future,
    net::SocketAddrV4,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct ProxyHandler {
    configuration: Arc<Configuration>,
}

impl ProxyHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

impl tower::Service<hyper::Request<Incoming>> for ProxyHandler {
    type Response = hyper::Response<Incoming>;
    type Error = ProxyError;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: hyper::Request<Incoming>) -> Self::Future {
        let host = request
            .headers()
            .get("Host")
            .map(|h| h.to_str().unwrap())
            .unwrap();

        let address = self
            .configuration
            .get()
            .applications
            .get(host)
            .unwrap()
            .address;

        Box::pin(async move { fetch(address, request).await })
    }
}

pub async fn fetch(
    address: SocketAddrV4,
    request: Request<Incoming>,
) -> Result<Response<Incoming>, ProxyError> {
    let stream = TcpStream::connect(address)
        .await
        .map_err(|_e| ProxyError::Connection)?;

    let io = TokioIo::new(stream);

    let (mut sender, connection) = hyper::client::conn::http1::handshake(io)
        .await
        .map_err(|_e| ProxyError::Handshake)?;

    // Spawn a task to poll the connection, driving the HTTP state
    let driver = tokio::spawn(connection);

    let response = sender
        .send_request(request)
        .await
        .map_err(|_e| ProxyError::Send)?;

    if let Err(_e) = driver.await.unwrap() {
        return Err(ProxyError::Completion);
    };

    Ok(response)
}

#[derive(Debug)]
pub enum ProxyError {
    Connection,
    Completion,
    Handshake,
    Send,
}

pub enum ProxyBody {
    Error(Full<Bytes>),
    Incoming(Incoming),
}

impl HyperBody for ProxyBody {
    type Data = Bytes;
    type Error = ();

    fn poll_frame(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, ()>>> {
        match self.get_mut() {
            ProxyBody::Error(body) => Pin::new(body).poll_frame(context).map_err(|_| ()),
            ProxyBody::Incoming(body) => Pin::new(body).poll_frame(context).map_err(|_| ()),
        }
    }
}
