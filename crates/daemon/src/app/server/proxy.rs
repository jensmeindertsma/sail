use crate::configuration::Configuration;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Response, StatusCode,
};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct ProxyHandler {
    configuration: Arc<Configuration>,
}

impl ProxyHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

#[derive(Clone, Debug)]
pub struct ProxySettings {
    pub hostname: String,
}

impl tower::Service<hyper::Request<Incoming>> for ProxyHandler {
    type Response = hyper::Response<Full<Bytes>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: hyper::Request<Incoming>) -> Self::Future {
        Box::pin(async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Full::new(Bytes::from(format!(
                    "<h1>Proxy {} {}</h1>",
                    request
                        .headers()
                        .get("Host")
                        .map(|v| v.to_str().unwrap_or("<invalid host>"))
                        .unwrap_or("<no host>"),
                    request.uri()
                ))))
                .unwrap())
        })
    }
}
