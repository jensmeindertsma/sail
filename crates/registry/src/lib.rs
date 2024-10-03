use axum::{
    body::HttpBody,
    http::{Request, StatusCode},
    response::{Html, Response},
    routing::{future::RouteFuture, get},
    Router,
};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::Service;
use tower_http::trace::TraceLayer;

pub struct Registry {
    router: Router,
}

impl Registry {
    pub fn new() -> Self {
        let router = Router::new()
            .layer(TraceLayer::new_for_http().on_request(()))
            .route(
                "/v2/",
                get((
                    StatusCode::OK,
                    Html("<h1>Registry API v2 is supported!</h1>"),
                )),
            )
            .fallback((StatusCode::NOT_FOUND, Html("<h1>Not Found</h1>")));

        Self { router }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
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
