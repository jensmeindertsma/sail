use core::fmt::{self, Formatter};
use hyper::{body::Incoming, Request, Response};
use hyper_util::rt::TokioIo;
use std::{error::Error, io, net::SocketAddrV4};
use tokio::net::TcpStream;
use tracing::{error, info, instrument, Instrument};

#[instrument(name = "proxy", skip_all)]
pub async fn proxy_request(
    request: Request<Incoming>,
    address: SocketAddrV4,
) -> Result<Response<Incoming>, ProxyError> {
    let stream = TcpStream::connect(address)
        .await
        .map_err(ProxyError::Connect)?;
    let io = TokioIo::new(stream);

    info!("connecting to {address}");

    let (mut sender, connection) = hyper::client::conn::http1::handshake(io)
        .await
        .map_err(ProxyError::Handshake)?;

    tokio::spawn(
        async move {
            if let Err(error) = connection.await {
                error!("proxy connection failed: {error}",);
            }
        }
        .in_current_span(),
    );

    info!("forwarding request to {} to application", request.uri());

    sender
        .send_request(request)
        .await
        .map_err(ProxyError::SendRequest)
}

#[derive(Debug)]
pub enum ProxyError {
    Connect(io::Error),
    Handshake(hyper::Error),
    SendRequest(hyper::Error),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connect(error) => write!(f, "proxy failed to connect: {error}"),
            Self::Handshake(error) => write!(f, "proxy handshake failed: {error}"),
            Self::SendRequest(error) => write!(f, "proxy failed to send request: {error}"),
        }
    }
}

impl Error for ProxyError {}
