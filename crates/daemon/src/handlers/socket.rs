use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use sail_core::{ConfigureError, SocketRequest, SocketResponse, Status};
use tower::Service;

use crate::configuration::{Configuration, Settings};

#[derive(Clone)]
pub struct SocketHandler {
    configuration: Arc<Configuration>,
}

impl SocketHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

impl Service<SocketRequest> for SocketHandler {
    type Response = SocketResponse;
    type Error = Infallible;
    type Future = SocketHandlerFuture;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        SocketHandlerFuture {
            configuration: self.configuration.clone(),
            request,
        }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct SocketHandlerFuture {
    configuration: Arc<Configuration>,
    request: SocketRequest,
}

impl Future for SocketHandlerFuture {
    type Output = Result<SocketResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let response = match &self.request {
            SocketRequest::Configure { setting, value } => match setting.as_str() {
                "greeting" => {
                    self.configuration.set(Settings {
                        greeting: value.to_owned(),
                    });
                    SocketResponse::Configure(Ok(()))
                }
                _ => SocketResponse::Configure(Err(ConfigureError::UnknownSetting(
                    setting.to_owned(),
                ))),
            },
            SocketRequest::Status => SocketResponse::Status(Status {
                // TODO: pull actual values from configuration here.
                dashboard_hostname: "foo".to_owned(),
                registry_hostname: "bar".to_owned(),
            }),
        };

        Poll::Ready(Ok(response))
    }
}
