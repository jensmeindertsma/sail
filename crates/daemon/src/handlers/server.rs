use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tracing::info;

use crate::configuration::Configuration;

#[derive(Clone)]
pub struct ServerHandler {
    configuration: Arc<Configuration>,
}

impl ServerHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

impl Service<Request<Incoming>> for ServerHandler {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = ServerHandlerFuture;

    fn call(&mut self, request: Request<Incoming>) -> Self::Future {
        ServerHandlerFuture {
            configuration: self.configuration.clone(),
            request,
        }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct ServerHandlerFuture {
    configuration: Arc<Configuration>,
    request: Request<Incoming>,
}

impl Future for ServerHandlerFuture {
    type Output = Result<Response<Full<Bytes>>, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let greeting = self.configuration.get().greeting;

        info!("handling request to {}", self.request.uri());

        Poll::Ready(Ok(Response::new(Full::new(Bytes::from(format!(
            "{greeting} `{}`\n",
            self.request.uri()
        ))))))
    }
}
