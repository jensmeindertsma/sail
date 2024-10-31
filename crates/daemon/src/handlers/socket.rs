use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use sail_core::{ConfigureError, SocketRequest, SocketResponse, Status};
use tower::Service;

use crate::configuration::{Configuration, DashboardSettings, RegistrySettings, Settings};

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
            SocketRequest::Configure { setting, value } => {
                let result = configure(&self.configuration, setting, value);

                SocketResponse::Configure(result)
            }
            SocketRequest::Status => {
                let settings = self.configuration.get();
                let status = Status {
                    dashboard_hostname: settings.dashboard.hostname,
                    registry_hostname: settings.registry.hostname,
                };

                SocketResponse::Status(status)
            }
        };

        Poll::Ready(Ok(response))
    }
}

fn configure(
    configuration: &Arc<Configuration>,
    setting: &str,
    value: &str,
) -> Result<(), ConfigureError> {
    let current_settings = configuration.get();

    let mut parts = setting.split('.');

    match parts
        .next()
        .ok_or(ConfigureError::UnknownSetting(setting.to_owned()))?
    {
        "dashboard" => {
            match parts
                .next()
                .ok_or(ConfigureError::UnknownSetting(setting.to_owned()))?
            {
                "hostname" => {
                    configuration.set(Settings {
                        dashboard: DashboardSettings {
                            hostname: value.to_owned(),
                        },
                        ..current_settings
                    });
                }
                other => Err(ConfigureError::UnknownSetting(other.to_owned()))?,
            }
        }

        "greeting" => {
            configuration.set(Settings {
                greeting: value.to_owned(),
                ..current_settings
            });
        }

        "registry" => {
            match parts
                .next()
                .ok_or(ConfigureError::UnknownSetting(setting.to_owned()))?
            {
                "hostname" => {
                    configuration.set(Settings {
                        registry: RegistrySettings {
                            hostname: value.to_owned(),
                        },
                        ..current_settings
                    });
                }
                other => Err(ConfigureError::UnknownSetting(other.to_owned()))?,
            }
        }

        other => Err(ConfigureError::UnknownSetting(other.to_owned()))?,
    }

    Ok(())
}
