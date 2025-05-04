mod configuration;
mod shutdown;
mod socket;

use configuration::Configuration;
use sail_core::socket::{SocketRequest, SocketResponse};
use shutdown::setup_shutdown_handler;
use socket::{AttachmentError, Socket};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    process::{ExitCode, Termination},
    sync::Arc,
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    task::JoinHandle,
};
use tracing::{Instrument, Level, info_span, instrument::Instrumented};

#[tokio::main]
async fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let mut shutdown_signal = setup_shutdown_handler();

    let configuration = Arc::new(Configuration::load().unwrap());

    tracing::info!("loaded configuration: {:#?}", configuration.get());

    let socket_cfg = configuration.clone();
    let socket_task: Instrumented<JoinHandle<Result<(), SocketTaskError>>> =
        tokio::spawn(async move {
            let listener = Socket::attach()
                .await
                .map_err(SocketTaskError::Attachment)?;

            loop {
                let (stream, _socket_address) = listener
                    .accept()
                    .await
                    .map_err(SocketTaskError::Accepting)?;

                tracing::info!("accepted new connection");

                // We spawn a parent task that will then spawn the connection
                // handler into another task. This allows us to monitor the
                // handler task and report an error if the handler panics or
                // returns an error.
                let parent_cfg = socket_cfg.clone();
                tokio::spawn(async move {
                    let handler_cfg = parent_cfg.clone();
                    let handle: Instrumented<JoinHandle<Result<(), SocketTaskError>>> =
                        tokio::spawn(async move {
                            tracing::info!("handling new connection");

                            let (reader, mut writer) = stream.into_split();
                            let mut reader = BufReader::new(reader).lines();

                            loop {
                                let request: SocketRequest = serde_json::from_str(&match reader
                                    .next_line()
                                    .await
                                    .map_err(SocketTaskError::Read)?
                                {
                                    None => {
                                        tracing::info!("no message, finished handling connection");
                                        return Err(SocketTaskError::NoMessage);
                                    }
                                    Some(message) => message,
                                })
                                .map_err(SocketTaskError::Deserialization)?;

                                // TODO create Handler here which implements service and get response that way
                                let response = match request {
                                    SocketRequest::SetGreeting { message } => {
                                        let mut settings = handler_cfg.get();
                                        settings.greeting = message;
                                        handler_cfg.set(settings);

                                        SocketResponse::Success
                                    }
                                    SocketRequest::Greet { message } => {
                                        SocketResponse::Greeting { message }
                                    }
                                };

                                writer
                                    .write_all(
                                        format!(
                                            "{}\n",
                                            serde_json::to_string(&response)
                                                .map_err(SocketTaskError::Serialization)?
                                        )
                                        .as_bytes(),
                                    )
                                    .await
                                    .map_err(SocketTaskError::Send)?;
                            }
                        })
                        .instrument(info_span!("handler"));

                    match handle.await {
                        Err(join_error) => {
                            tracing::error!(
                                "socket connection handler panicked or was cancelled: {join_error}"
                            )
                        }
                        Ok(output) => {
                            if let Err(error) = output {
                                tracing::warn!(
                                    "socket connection handler exited with error: {error}",
                                )
                            }
                        }
                    }
                });
            }
        })
        .instrument(info_span!("socket"));

    let mut crashed = false;

    tokio::select! {
        result = socket_task => {
            crashed = true;

            match result {
                Err(join_error) => tracing::error!("socket task panicked or was cancelled: {join_error}"),
                Ok(output) => match output {
                    Ok(_) => tracing::warn!("socket task exited cleanly unexpected"),
                    Err(error) => tracing::error!("socket task terminated unexpectedly with error: {error}"),
                }
            };
        }

        _ = shutdown_signal.changed() => {
            tracing::info!("received shutdown signal, initiating shutdown");
        }
    }

    if crashed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[derive(Debug)]
enum SocketTaskError {
    Accepting(io::Error),
    Attachment(AttachmentError),
    Deserialization(serde_json::Error),
    NoMessage,
    Read(io::Error),
    Send(io::Error),
    Serialization(serde_json::Error),
}

impl Display for SocketTaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepting(error) => write!(f, "failed to accept connection: {error}"),
            Self::Attachment(error) => write!(f, "failed to attach to socket: {error}"),
            Self::Deserialization(error) => {
                write!(f, "failed to deserialize incoming message: {error}")
            }
            Self::NoMessage => write!(f, "received no message from new connection"),
            Self::Read(error) => write!(f, "failed to read incoming message: {error}"),
            Self::Send(error) => write!(f, "failed to send outgoing message: {error}"),
            Self::Serialization(error) => {
                write!(f, "failed to serialize outgoing message: {error}")
            }
        }
    }
}

impl Error for SocketTaskError {}
