mod configuration;
mod handlers;
mod server;
mod shutdown;
mod socket;

use configuration::{Configuration, LoadError};
use core::fmt::{self, Formatter};
use handlers::{ServerHandler, SocketHandler};
use hyper_util::server::graceful::GracefulShutdown;
use server::Server;
use shutdown::setup_shutdown_listener;
use socket::{AttachmentError, Socket};
use std::{error::Error, io, process::ExitCode, sync::Arc};
use tokio::time::{sleep, Duration};
use tracing::{error, info, info_span, Instrument, Level};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    if let Err(failure) = run().await {
        error!("{failure}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run() -> Result<(), Failure> {
    let mut shutdown_signal = setup_shutdown_listener();

    let configuration = Arc::new(Configuration::load().await?);

    let socket = Socket::attach()?;

    info!("attached to the socket");

    let socket_configuration = configuration.clone();
    let socket_task = tokio::spawn(
        async move {
            socket
                .serve_connections(SocketHandler::new(socket_configuration))
                .await;
        }
        .instrument(info_span!("socket")),
    );

    let server = Server::bind("127.0.0.1:4250")
        .await
        .map_err(Failure::ServerBindError)?;

    info!("listening on `127.0.0.1:4250`");

    let watcher = GracefulShutdown::new();
    let mut socket_stopped_unexpectedly = false;

    tokio::select! {
        biased;

        _ = socket_task => {
            error!("socket task stopped unexpectedly, shutting down!");
            socket_stopped_unexpectedly = true
        }

        _ = shutdown_signal.changed() => {}

        _ = server
            .serve_connections(
                ServerHandler::new(configuration),
                &watcher
            )
            .instrument(info_span!("server")) => {},
    };

    tokio::select! {
        // Calling `.shutdown()` on the handle returned by the server after it stops its connection
        // loop, will gracefully terminate current connections and return a future that resolves
        // when this is finished.
        _ = watcher.shutdown() => {
            info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            error!("timed out wait for all connections to close");
        }
    };

    if socket_stopped_unexpectedly {
        return Err(Failure::SocketStoppedUnexpectedly);
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    CannotLoadConfiguration(LoadError),
    ServerBindError(io::Error),
    SocketAttachment(AttachmentError),
    SocketStoppedUnexpectedly,
}

impl From<LoadError> for Failure {
    fn from(error: LoadError) -> Self {
        Self::CannotLoadConfiguration(error)
    }
}

impl From<AttachmentError> for Failure {
    fn from(error: AttachmentError) -> Self {
        Self::SocketAttachment(error)
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotLoadConfiguration(load_error) => {
                write!(f, "failed to load configuration: {load_error}")
            }
            Self::ServerBindError(io_error) => {
                write!(f, "failed to bind server listener: {io_error}")
            }
            Self::SocketAttachment(attachment_error) => {
                write!(f, "failed to attach to socket: {attachment_error}")
            }
            Self::SocketStoppedUnexpectedly => write!(f, "socket handler stopped unexpectedly"),
        }
    }
}

impl Error for Failure {}
