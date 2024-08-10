use crate::configuration::Configuration;
use axum::{
    extract::Request as AxumRequest, response::Response as AxumResponse,
    routing::future::RouteFuture, Router as AxumRouter,
};
use hyper::body::Incoming;
use std::{
    convert::Infallible,
    sync::Arc,
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct InterfaceHandler {
    router: AxumRouter<()>,
    configuration: Arc<Configuration>,
}

impl InterfaceHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self {
            router: AxumRouter::new(),
            configuration,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InterfaceSettings {
    pub hostname: String,
}

impl tower::Service<AxumRequest<Incoming>> for InterfaceHandler {
    type Response = AxumResponse;
    type Error = Infallible;
    type Future = RouteFuture<Infallible>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <AxumRouter as tower::Service<AxumRequest>>::poll_ready(&mut self.router, context)
    }

    fn call(&mut self, request: AxumRequest<Incoming>) -> Self::Future {
        self.router.call(request)
    }
}
