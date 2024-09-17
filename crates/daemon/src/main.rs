mod configuration;
mod socket;

use configuration::Configuration;
use sail_core::{
    configuration::Application,
    socket::{FailureReason, SocketReply, SocketRequest, SocketResponse, SuccessResponse},
};
use socket::Socket;
use std::{process::ExitCode, sync::Arc};
use tracing::{error, info, info_span, Instrument, Level};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    info!("starting up...");

    let configuration = Arc::new(Configuration::load());

    info!("loaded configuration: {:?}", configuration.get());

    let mut socket = match Socket::attach() {
        Ok(socket) => socket,
        Err(error) => {
            error!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let span = info_span!("socket");
    let guard = span.enter();
    while let Some(mut connection) = socket.accept().await {
        let configuration = configuration.clone();
        tokio::spawn(
            async move {
                // Wait for incoming messages over the connection, then handle their request,
                // and reply with an appropriate response.
                while let Some(message) = connection.next_message().await {
                    info!(
                        "received message #{} with request {:?}",
                        message.id, message.request
                    );

                    let response = match message.request {
                        SocketRequest::ListApplications => SocketResponse::Success(
                            SuccessResponse::ListApplications(configuration.get().applications),
                        ),

                        SocketRequest::CreateApplication {
                            name,
                            hostname,
                            address,
                        } => {
                            let mut settings = configuration.get();

                            if settings.applications.iter().any(|app| app.name == name) {
                                SocketResponse::Failure(FailureReason::NameInUse)
                            } else {
                                settings.applications.push(Application {
                                    name: name.clone(),
                                    hostname,
                                    address,
                                });

                                configuration.set(settings);

                                SocketResponse::Success(SuccessResponse::CreatedApplication {
                                    name,
                                })
                            }
                        }

                        _ => SocketResponse::Failure(FailureReason::Todo),
                    };

                    info!(
                        "replying to message #{} with response {:?}",
                        message.id, response
                    );

                    connection
                        .reply(SocketReply {
                            regarding: message.id,
                            response,
                        })
                        .await
                }
            }
            .instrument(span.clone()),
        );
    }
    drop(guard);

    ExitCode::SUCCESS
}
