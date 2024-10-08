mod routes;

use axum::{
    body::HttpBody,
    http::Request,
    response::Response,
    routing::{future::RouteFuture, get, patch, post, put},
    Router,
};
use sail_core::configuration::Configuration;
use std::{
    convert::Infallible,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tower_http::trace::TraceLayer;

pub struct Registry {
    router: Router,
}

impl Registry {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        let router = Router::new()
            .route("/v2/", get(routes::version_check))
            .route("/v2/:name/blobs/uploads/", post(routes::initiate_upload))
            .route("/v2/:name/blobs/uploads/:uuid", patch(routes::upload_blob))
            .route(
                "/v2/:name/blobs/uploads/:uuid",
                put(routes::complete_upload),
            )
            .route(
                "/v2/:name/manifests/:reference",
                put(routes::upload_manifest),
            )
            .with_state(configuration)
            .fallback(routes::not_found)
            .layer(TraceLayer::new_for_http().on_request(()));

        Self { router }
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
