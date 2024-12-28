use crate::configuration::Configuration;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use std::{
    convert::Infallible,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tracing::info;

#[derive(Clone, Debug)]
pub struct ServerHandler {
    configuration: Arc<Configuration>,
}

impl ServerHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

type ServerRequest = hyper::Request<Incoming>;
type ServerResponse = hyper::Response<Full<Bytes>>;

impl Service<ServerRequest> for ServerHandler {
    type Error = Infallible;
    type Future = ServerFuture;
    type Response = ServerResponse;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: ServerRequest) -> Self::Future {
        info!("handling request to {}", request.uri());

        ServerFuture {
            request,
            configuration: self.configuration.clone(),
        }
    }
}

pub struct ServerFuture {
    request: ServerRequest,
    configuration: Arc<Configuration>,
}

impl Future for ServerFuture {
    type Output = Result<ServerResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let greeting = self.configuration.get().greeting;
        let uri = self.request.uri();

        let response = hyper::Response::new(Full::new(Bytes::from(format!(
            "<h1>Welcome!</h1><p>{greeting}</p><p>{uri}</p>",
        ))));

        Poll::Ready(Ok(response))
    }
}
