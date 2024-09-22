use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{
    body::HttpBody,
    http::Request,
    response::{Html, Response},
    routing::{future::RouteFuture, get},
    Router,
};
use tower::Service;

pub struct Registry {
    router: Router,
}

impl Registry {
    pub fn new() -> Self {
        let router = Router::new().route("/", get(hello_world));

        Self { router }
    }
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Registry says 'Hello, World!'</h1>")
}

pub type RegistryFuture = RouteFuture<Infallible>;

impl<B> Service<Request<B>> for Registry
where
    B: HttpBody<Data = axum::body::Bytes> + Send + 'static,
    B::Error: std::error::Error + Send + Sync,
{
    type Response = Response;
    type Error = Infallible;
    type Future = RouteFuture<Infallible>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <Router as Service<Request<B>>>::poll_ready(&mut self.router, context)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        self.router.call(request)
    }
}
