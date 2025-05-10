mod configuration;
mod shutdown;
mod socket;

use configuration::Configuration;
use shutdown::setup_shutdown_handler;
use socket::{AttachmentError, Socket, connection::SocketConnection};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    process::{ExitCode, Termination},
    sync::Arc,
};
use tokio::{io, task::JoinHandle};
use tracing::{Instrument, Level, info_span, instrument::Instrumented};

#[tokio::main]
async fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let mut shutdown_signal = setup_shutdown_handler();

    let configuration = match Configuration::load() {
        Ok(configuration) => Arc::new(configuration),
        Err(error) => {
            tracing::error!("failed to load configuration: {error}");
            return ExitCode::FAILURE;
        }
    };

    tracing::info!("loaded configuration");

    // We spawn a task here that is responsible for accepting and handling
    // new connections and messages coming in over the socket.
    let socket_cfg = configuration.clone();
    let socket_task: Instrumented<JoinHandle<Result<(), SocketTaskError>>> =
        tokio::spawn(async move {
            let listener = Socket::attach()
                .await
                .map_err(SocketTaskError::Attachment)?;

            // The socket connection acceptance loop
            loop {
                let (stream, _socket_address) = listener
                    .accept()
                    .await
                    .map_err(SocketTaskError::Accepting)?;

                tracing::info!("handling new connection");

                let handler_cfg = socket_cfg.clone();
                let handler_task = tokio::spawn(async move {
                    SocketConnection::new(handler_cfg, stream).serve().await
                })
                .instrument(info_span!("handler"));

                // We must spawn a task here that deals with
                // the handler exiting due to a panic or
                // error. If we don't spawn we would block
                // the connection acceptance loop.
                tokio::spawn(async move {
                    match handler_task.await {
                        Err(join_error) => {
                            tracing::error!(
                                "connection handler panicked or was cancelled: {join_error}"
                            )
                        }
                        Ok(Err(connection_error)) => {
                            tracing::warn!(
                                "connection handler exited with error: {connection_error}",
                            )
                        }
                        Ok(Ok(())) => {}
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
}

impl Display for SocketTaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepting(error) => write!(f, "failed to accept connection: {error}"),
            Self::Attachment(error) => write!(f, "failed to attach to socket: {error}"),
        }
    }
}

impl Error for SocketTaskError {}
