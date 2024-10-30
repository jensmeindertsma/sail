use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use sail_core::{ConfigureError, SocketRequest, SocketResponse, Status};
use tower::Service;
use tracing::info;

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
        let response = match &self.request {
            SocketRequest::Configure { setting, value } => {
                info!("configuring setting `{setting}` to `{value}`");

                SocketResponse::Configure(Err(ConfigureError::UnknownSetting))
            }
            SocketRequest::Status => SocketResponse::Status(Status {
                // TODO: pull actual values from configuration here.
                dashboard_hostname: "foo".to_owned(),
                registry_hostname: "bar".to_owned(),
            }),
        };

        Poll::Ready(Ok(response))
    }
}
