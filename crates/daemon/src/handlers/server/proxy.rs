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
    let stream = TcpStream::connect(address).await.map_err(|e| {
        error!(destination = address.to_string(), "failed to connect");
        ProxyError::Connect(e)
    })?;

    let io = TokioIo::new(stream);

    info!("connected to {address}");

    let (mut sender, connection) =
        hyper::client::conn::http1::handshake(io)
            .await
            .map_err(|e| {
                error!("handshake failed");
                ProxyError::Handshake(e)
            })?;

    tokio::spawn(
        async move {
            if let Err(error) = connection.await {
                error!("proxy connection failed: {error}",);
            }
        }
        .in_current_span(),
    );

    info!("sending request");

    sender.send_request(request).await.map_err(|e| {
        error!("error while sending request");
        ProxyError::SendRequest(e)
    })
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
