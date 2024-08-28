use axum::{
    extract::Request as AxumRequest, response::Response as AxumResponse,
    routing::future::RouteFuture, Router as AxumRouter,
};
use hyper::{body::Incoming, StatusCode, Uri};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct InterfaceHandler {
    router: AxumRouter<()>,
}

impl InterfaceHandler {
    pub fn new() -> Self {
        Self {
            router: AxumRouter::new().fallback(fallback),
        }
    }
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("INTERFACE No route for {uri}"),
    )
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
