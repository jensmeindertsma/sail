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
    io::{self},
    process::{ExitCode, Termination},
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
    task::JoinHandle,
};
use tracing::{Instrument, Level, info, info_span, instrument::Instrumented};

#[tokio::main]
async fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let mut shutdown_signal = setup_shutdown_handler();

    let configuration = Arc::new(Configuration::load().unwrap());

    tracing::info!("loaded configuration: {:#?}", configuration.get());

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
                tokio::spawn(async move {
                    tracing::info!("spawning new task to handle connected");
                    let result = tokio::spawn(handle_socket_connection(stream))
                        .instrument(info_span!("handler"))
                        .await;

                    match result {
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
        biased;

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

    // tokio::select! {
    //     _ = connection_watcher.shutdown() => {
    //         tracing::info!("all connections gracefully closed");
    //     },
    //     _ = sleep(Duration::from_secs(10)) => {
    //         tracing::error!("timed out wait for all connections to close");
    //     }
    // };

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

async fn handle_socket_connection(stream: UnixStream) -> Result<(), SocketTaskError> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader).lines();

    loop {
        let request: SocketRequest =
            match reader.next_line().await.map_err(SocketTaskError::Read)? {
                None => return Err(SocketTaskError::NoMessage),
                Some(message) => {
                    serde_json::from_str(&message).map_err(SocketTaskError::Deserialization)?
                }
            };

        let response = match request {
            SocketRequest::Greet { message } => {
                info!("received message: {message}");
                SocketResponse::Greeting {
                    message: "Thanks for your message. I hope you will have a great day!"
                        .to_owned(),
                }
            }
        };

        writer
            .write_all(
                format!(
                    "{}\n",
                    serde_json::to_string(&response).map_err(SocketTaskError::Serialization)?
                )
                .as_bytes(),
            )
            .await
            .map_err(SocketTaskError::Send)?;
    }
}
