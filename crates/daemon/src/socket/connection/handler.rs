use crate::configuration::{Configuration, Settings};
use sail_core::socket::{SocketRequest, SocketResponse};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;

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
    type Error = HandlerError;
    type Future = HandlerFuture;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        HandlerFuture {
            configuration: self.configuration.clone(),
            request,
        }
    }

    fn poll_ready(&mut self, _context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct HandlerFuture {
    configuration: Arc<Configuration>,
    request: SocketRequest,
}

impl Future for HandlerFuture {
    type Output = Result<SocketResponse, HandlerError>;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        let response = match &self.request {
            SocketRequest::Greet { message } => {
                tracing::info!("received greeting from socket connection: {message}");
                Ok(SocketResponse::Greeting {
                    message: self.configuration.get().greeting,
                })
            }
            SocketRequest::SetGreeting { message } => {
                tracing::info!("received request to modify greeting setting");
                let settings = self.configuration.get();
                tracing::debug!("fetched settings");

                let new_settings = Settings {
                    greeting: message.to_owned(),
                    ..settings
                };

                self.configuration.set(new_settings);

                tracing::info!("modified greeting setting to `{message}`");

                Ok(SocketResponse::Success)
            }
        };

        Poll::Ready(response)
    }
}

#[derive(Debug)]
pub enum HandlerError {}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
    }
}

impl Error for HandlerError {}
