use std::sync::Arc;

use sail_core::socket::{SocketRequest, SocketResponse};
use tower::Service;

use crate::configuration::Configuration;

pub struct SocketHandler {
    configuration: Arc<Configuration>,
}

impl SocketHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

impl Service<SocketRequest> for SocketHandler {
    type Response = SocketResponse;
    type Error = SocketHandlerError;
    type Future = SocketHandlerFuture;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        SocketHandlerFuture {
            configuration: self.configuration.clone(),
        }
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}

pub enum SocketHandlerError {}

pub struct SocketHandlerFuture {
    configuration: Arc<Configuration>,
}

impl Future for SocketHandlerFuture {
    type Output = Result<SocketResponse, SocketHandlerError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
