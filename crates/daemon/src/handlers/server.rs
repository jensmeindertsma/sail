mod body;
mod proxy;

use crate::configuration::Configuration;
use body::Body;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode,
};
use pin_project::pin_project;
use proxy::{proxy_request, ProxyError};
use sail_core::configuration::Application;
use sail_dashboard::{Dashboard, DashboardFuture};
use sail_registry::{Registry, RegistryFuture};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tracing::{error, info};

#[derive(Clone)]
pub struct ServerHandler {
    configuration: Arc<Configuration>,
}

impl ServerHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

type ServerRequest = Request<Incoming>;
type ServerResponse = Response<Body>;

impl Service<ServerRequest> for ServerHandler {
    type Response = ServerResponse;
    type Error = Infallible;
    type Future = ServerHandlerFuture;

    fn call(&mut self, request: ServerRequest) -> Self::Future {
        // TODO: actual implementation of request forwarding based on headers

        let Some(host) = request.headers().get("host") else {
            error!(?request, "request is missing Host header");
            return ServerHandlerFuture::BadRequest(request);
        };

        let Ok(host) = host.to_str() else {
            error!(?request, "request has non UTF-8 Host header");
            return ServerHandlerFuture::BadRequest(request);
        };

        let settings = self.configuration.get();

        match host {
            host if host == settings.dashboard.hostname => {
                info!("forwarding request to dashboard");

                let mut dashboard = Dashboard::new();

                ServerHandlerFuture::Dashboard(dashboard.call(request))
            }
            host if host == settings.registry.hostname => {
                info!("forwarding request to registry");

                let mut registry = Registry::new();

                ServerHandlerFuture::Registry(registry.call(request))
            }

            host => {
                if let Some(app) = settings
                    .applications
                    .into_iter()
                    .find(|app| app.hostname == host)
                {
                    if let Some(address) = app.address {
                        info!(
                            "request is to application `{}`, proxying request to {}",
                            app.name, address,
                        );
                        ServerHandlerFuture::Proxy(Box::pin(proxy_request(request, address)))
                    } else {
                        info!(
                            "request is to application `{}`, it has no address!",
                            app.name,
                        );

                        ServerHandlerFuture::Placeholder(request, app)
                    }
                } else {
                    error!(
                        host = ?request.headers().get("host"),
                        "received request to unknown host"
                    );
                    ServerHandlerFuture::UnknownHost(request)
                }
            }
        }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

#[pin_project(project = Projected)]
pub enum ServerHandlerFuture {
    BadRequest(ServerRequest),
    Dashboard(#[pin] DashboardFuture),
    Placeholder(ServerRequest, Application),
    Proxy(#[pin] Pin<Box<dyn Future<Output = Result<Response<Incoming>, ProxyError>> + Send>>),
    Registry(#[pin] RegistryFuture),
    UnknownHost(ServerRequest),
}

impl Future for ServerHandlerFuture {
    type Output = Result<ServerResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            Projected::BadRequest(_request) => Poll::Ready(Ok(make_response(
                "400 Bad Request\n",
                StatusCode::BAD_REQUEST,
            )
            .map(Body::Complete))),

            Projected::Dashboard(future) => future
                .poll(context)
                .map(|result| result.map(|response| response.map(Body::Axum))),

            Projected::Placeholder(_request, app) => Poll::Ready(Ok(make_response(
                &format!("Placeholder page for app `{}`\n", app.name),
                StatusCode::OK,
            )
            .map(Body::Complete))),
            Projected::Proxy(future) => future.poll(context).map(|result| match result {
                Ok(response) => {
                    info!("proxy returned response");
                    Ok(response.map(Body::Incoming))
                }
                Err(_proxy_error) => {
                    error!(status = 502, "proxy failed, responding with error page");
                    Ok(
                        make_response("502 Bad Gateway (proxy error)\n", StatusCode::BAD_GATEWAY)
                            .map(Body::Complete),
                    )
                }
            }),

            Projected::Registry(future) => future
                .poll(context)
                .map(|result| result.map(|response| response.map(Body::Axum))),

            Projected::UnknownHost(_request) => Poll::Ready(Ok(make_response(
                "421 Unknown Host\n",
                StatusCode::MISDIRECTED_REQUEST,
            )
            .map(Body::Complete))),
        }
    }
}

fn make_response(body: &str, status: StatusCode) -> hyper::Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body.to_owned())))
        .expect("constructing hyper response should not fail")
}
