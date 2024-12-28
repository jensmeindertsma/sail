use crate::configuration::{Configuration, Settings};
use sail_core::{RestartRequired, SocketRequest, SocketResponse};
use std::{
    convert::Infallible,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tracing::info;

#[derive(Clone, Debug)]
pub struct SocketHandler {
    configuration: Arc<Configuration>,
}

impl SocketHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }
}

impl Service<SocketRequest> for SocketHandler {
    type Error = Infallible;
    type Future = SocketFuture;
    type Response = SocketResponse;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        info!("handling request: {request:?}");

        SocketFuture {
            request,
            configuration: self.configuration.clone(),
        }
    }

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct SocketFuture {
    request: SocketRequest,
    configuration: Arc<Configuration>,
}

impl Future for SocketFuture {
    type Output = Result<SocketResponse, Infallible>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        use SocketRequest as Request;
        use SocketResponse as Response;

        let settings = self.configuration.get();

        let response = match &self.request {
            Request::FetchGreeting => Response::FetchGreeting(Ok(settings.greeting)),
            Request::ModifyGreeting(new_greeting) => {
                self.configuration.set(Settings {
                    greeting: new_greeting.into(),
                    ..settings
                });

                Response::ModifyGreeting(Ok(()))
            }
            Request::FetchPort => Response::FetchPort(Ok(settings.port)),
            Request::ModifyPort(new_port) => {
                self.configuration.set(Settings {
                    port: *new_port,
                    ..settings
                });

                Response::ModifyPort(Ok(RestartRequired))
            }
        };

        info!("generated response: {response:?}");

        Poll::Ready(Ok(response))
    }
}
