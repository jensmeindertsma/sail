pub mod interface;
pub mod proxy;
pub mod registry;

use crate::configuration::Configuration;
use axum::body::Body as AxumBody;
use core::fmt::{self, Display};
use http_body_util::Full;
use hyper::{
    body::{Body as HyperBody, Bytes, Frame, Incoming},
    Request as HyperRequest, Response as HyperResponse, StatusCode,
};
use interface::InterfaceHandler;
use proxy::ProxyHandler;
use registry::RegistryHandler;
use std::{
    convert::Infallible,
    error::Error,
    fmt::Formatter,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct ServerHandler {
    interface_handler: InterfaceHandler,
    proxy_handler: ProxyHandler,
    registry_handler: RegistryHandler,
    configuration: Arc<Configuration>,
}

impl ServerHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self {
            interface_handler: InterfaceHandler::new(configuration.clone()),
            proxy_handler: ProxyHandler::new(configuration.clone()),
            registry_handler: RegistryHandler::new(configuration.clone()),
            configuration,
        }
    }
}

impl tower::Service<HyperRequest<Incoming>> for ServerHandler {
    type Response = HyperResponse<HandlerBody>;
    type Error = Infallible;
    type Future = ServerHandlerFuture;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut polled = Vec::new();

        polled.push(self.interface_handler.poll_ready(context));
        polled.push(self.proxy_handler.poll_ready(context));
        polled.push(self.registry_handler.poll_ready(context));

        if polled.iter().all(|p| p.is_ready()) {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, request: HyperRequest<Incoming>) -> Self::Future {
        let settings = self.configuration.get();
        let mut interface_handler = self.interface_handler.clone();
        let mut registry_handler = self.registry_handler.clone();
        let mut proxy_handler = self.proxy_handler.clone();
        Self::Future {
            response_future: Box::pin(async move {
                let response = match request.headers().get("Host") {
                    None => make_error_response("missing Host header").map(HandlerBody::Error),
                    Some(host) => {
                        let host = match host.to_str() {
                            Ok(str) => str,
                            Err(_error) => {
                                return Ok(make_error_response("invalid host header")
                                    .map(HandlerBody::Error))
                            }
                        };

                        if host == settings.interface.hostname {
                            interface_handler
                                .call(request)
                                .await?
                                .map(HandlerBody::Axum)
                        } else if host == settings.registry.hostname {
                            registry_handler.call(request).await?.map(HandlerBody::Axum)
                        } else if settings.applications.contains_key(host) {
                            proxy_handler.call(request).await?.map(HandlerBody::Proxy)
                        } else {
                            make_error_response("unknown host").map(HandlerBody::Error)
                        }
                    }
                };

                Ok(response)
            }),
        }
    }
}

struct ServerHandlerFuture {
    response_future: Pin<
        Box<dyn Future<Output = Result<HyperResponse<HandlerBody>, Infallible>> + Send + 'static>,
    >,
}

impl Future for ServerHandlerFuture {
    type Output = Result<HyperResponse<HandlerBody>, Infallible>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.response_future.as_mut().poll(context)
    }
}

enum HandlerBody {
    Axum(AxumBody),
    Error(Full<Bytes>),
    Hyper(Incoming),
    Proxy(Full<Bytes>),
}

impl HyperBody for HandlerBody {
    type Data = Bytes;
    type Error = BodyError;

    fn poll_frame(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, BodyError>>> {
        match self.get_mut() {
            HandlerBody::Axum(body) => Pin::new(body).poll_frame(context).map_err(BodyError::Axum),
            HandlerBody::Error(body) => Pin::new(body)
                .poll_frame(context)
                .map_err(|_| BodyError::Infallible),
            HandlerBody::Hyper(body) => {
                Pin::new(body).poll_frame(context).map_err(BodyError::Hyper)
            }
            HandlerBody::Proxy(body) => Pin::new(body)
                .poll_frame(context)
                .map_err(|_| BodyError::Infallible),
        }
    }
}

#[derive(Debug)]
pub enum BodyError {
    Axum(axum::Error),
    Infallible,
    Hyper(hyper::Error),
}

impl Display for BodyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BodyError::Axum(e) => format!("axum body error: {e:?}"),
                BodyError::Infallible => format!("infallible error:?"),
                BodyError::Hyper(e) => format!("hyper body error: {e:?}"),
            }
        )
    }
}

impl Error for BodyError {}

fn make_error_response(message: &str) -> hyper::Response<Full<Bytes>> {
    HyperResponse::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/html")
        .body(Full::new(Bytes::from(format!(
            "<h1>Bad request!</h1><p>{message}</p>\n"
        ))))
        .unwrap()
}
