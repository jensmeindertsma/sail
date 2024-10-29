use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use sail_core::{SocketRequest, SocketResponse, Status};
use tower::Service;

#[derive(Clone)]
pub struct SocketHandler;

impl SocketHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Service<SocketRequest> for SocketHandler {
    type Response = SocketResponse;
    type Error = Infallible;
    type Future = SocketHandlerFuture;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        SocketHandlerFuture { request }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct SocketHandlerFuture {
    request: SocketRequest,
}

impl Future for SocketHandlerFuture {
    type Output = Result<SocketResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let response = match self.request {
            SocketRequest::Status => SocketResponse::Status(Status {
                listening_on: "127.0.0.1:4250".parse().unwrap(),
                dashboard_hostname: "foo".to_owned(),
                registry_hostname: "bar".to_owned(),
            }),
        };

        Poll::Ready(Ok(response))
    }
}
