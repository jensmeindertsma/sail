use crate::socket::SocketConnection;
use sail_core::{
    configuration::{Application, Configuration},
    socket::{Failure, SocketReply, SocketRequest, SocketResponse, Success},
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
use uuid::Uuid;

pub struct SocketHandler {
    configuration: Arc<Configuration>,
}

impl SocketHandler {
    pub fn new(configuration: Arc<Configuration>) -> Self {
        Self { configuration }
    }

    pub async fn serve_connection(&mut self, mut connection: SocketConnection) {
        while let Some(message) = connection.next_message().await {
            info!(id = message.id, "request = {:?}", message.request);

            let Ok(response) = self.call(message.request).await;

            info!(id = message.id, "response = {:?}", response);

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
        // This is always the latest copy as this method is invoked for
        // every request as least once.
        let mut settings = self.configuration.get();
        let response = match self.request.clone() {
            SocketRequest::CreateApplication { name, hostname } => {
                if settings.applications.iter().any(|app| app.name == name) {
                    Err(Failure::NameInUse)
                } else {
                    let app = Application {
                        name: name.clone(),
                        hostname: hostname.clone(),
                        address: None,
                        token: Uuid::new_v4().to_string(),
                    };
                    info!(hostname, address = ?app.address, "created application `{name}`",);

                    settings.applications.push(app);
                    self.configuration.set(settings);

                    Ok(Success::CreatedApplication)
                }
            }
            SocketRequest::DeleteApplication { name } => {
                settings.applications.retain(|app| app.name != name);
                self.configuration.set(settings);

                Ok(Success::DeletedApplication)
            }
            SocketRequest::EditApplication {
                name,
                new_name,
                new_hostname,
            } => {
                settings.applications = settings
                    .applications
                    .into_iter()
                    .map(|app| {
                        if app.name == name {
                            Application {
                                name: new_name.clone().unwrap_or(app.name),
                                hostname: new_hostname.clone().unwrap_or(app.hostname),
                                address: app.address,
                                token: app.token,
                            }
                        } else {
                            app
                        }
                    })
                    .collect();
                self.configuration.set(settings);

                Ok(Success::EditedApplication)
            }
            SocketRequest::EditDashboardHost { hostname } => {
                settings.dashboard.hostname = hostname;
                self.configuration.set(settings);
                Ok(Success::EditedDashboardHost)
            }
            SocketRequest::EditRegistryHost { hostname } => {
                settings.registry.hostname = hostname;
                self.configuration.set(settings);
                Ok(Success::EditedRegistryHost)
            }
            SocketRequest::GetApplication { name } => {
                if let Some(app) = settings
                    .applications
                    .into_iter()
                    .find(|app| app.name == name)
                {
                    Ok(Success::GetApplication(app))
                } else {
                    Err(Failure::ApplicationNotFound)
                }
            }
            SocketRequest::GetApplications => Ok(Success::GetApplications(settings.applications)),
            SocketRequest::GetDashboardHost => Ok(Success::GetDashboardHost {
                hostname: settings.dashboard.hostname,
            }),
            SocketRequest::GetRegistryHost => Ok(Success::GetRegistryHost {
                hostname: settings.registry.hostname,
            }),
        };

        Poll::Ready(Ok(response))
    }
}
