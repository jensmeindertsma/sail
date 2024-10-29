use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

#[derive(Clone)]
pub struct ServerHandler;

impl ServerHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Service<Request<Incoming>> for ServerHandler {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = ServerHandlerFuture;

    fn call(&mut self, request: Request<Incoming>) -> Self::Future {
        ServerHandlerFuture { request }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

struct ServerHandlerFuture {
    request: Request<Incoming>,
}

impl Future for ServerHandlerFuture {
    type Output = Result<Response<Full<Bytes>>, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(Response::new(Full::new(Bytes::from(format!(
            "Hello, World! `{}`",
            self.request.uri()
        ))))))
    }
}
