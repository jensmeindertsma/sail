use http_body_util::Full;
use hyper::body::Bytes;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::info;

#[derive(Clone)]
pub struct ServerHandler;

impl ServerHandler {
    pub fn new() -> Self {
        Self
    }
}

type ServerRequest = hyper::Request<hyper::body::Incoming>;

type ServerResponse = hyper::Response<Full<Bytes>>;

impl tower::Service<ServerRequest> for ServerHandler {
    type Response = ServerResponse;
    type Error = Infallible;
    type Future = ServerHandlerFuture;

    fn call(&mut self, request: ServerRequest) -> Self::Future {
        // TODO: actual implementation of request forwarding based on headers

        info!("handling request to {}", request.uri());

        ServerHandlerFuture { request }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct ServerHandlerFuture {
    request: ServerRequest,
}

impl Future for ServerHandlerFuture {
    type Output = Result<ServerResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(hyper::Response::new(Full::new(Bytes::from(format!(
            "Hello, World! You are on {}\n",
            self.request.uri().to_string()
        ))))))
    }
}
