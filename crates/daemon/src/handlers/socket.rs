use crate::{configuration::Configuration, socket::SocketConnection};
use sail_core::{
    configuration::Application,
    socket::{Reason, Requested, SocketReply, SocketRequest, SocketResponse},
};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;
use tracing::info;

pub struct SocketHandler {
    configuration: Arc<Configuration>,
}

impl SocketHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }

    pub async fn serve_connection(&mut self, mut connection: SocketConnection) {
        while let Some(message) = connection.next_message().await {
            info!(
                "received message #{} with request {:?}",
                message.id, message.request
            );

            let Ok(response) = self.call(message.request).await;

            info!(
                "replying to message #{} with response {:?}",
                message.id, response
            );

            connection
                .reply(SocketReply {
                    regarding: message.id,
                    response,
                })
                .await;
        }
    }
}

impl tower::Service<SocketRequest> for SocketHandler {
    type Response = SocketResponse;
    type Error = Infallible;
    type Future = SocketHandlerFuture;

    fn call(&mut self, request: SocketRequest) -> Self::Future {
        let configuration = self.configuration.clone();

        SocketHandlerFuture {
            configuration,
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
        let response = match self.request.clone() {
            SocketRequest::ListApplications => SocketResponse::Success(
                Requested::ListApplications(self.configuration.get().applications),
            ),

            SocketRequest::CreateApplication {
                name,
                hostname,
                address,
            } => {
                let mut settings = self.configuration.get();

                if settings.applications.iter().any(|app| app.name == name) {
                    SocketResponse::Failure(Reason::NameInUse)
                } else {
                    settings.applications.push(Application {
                        name: name.clone(),
                        hostname,
                        address,
                    });

                    self.configuration.set(settings);

                    SocketResponse::Success(Requested::CreatedApplication { name })
                }
            }

            _ => SocketResponse::Failure(Reason::Todo),
        };

        Poll::Ready(Ok(response))
    }
}
