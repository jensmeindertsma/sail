mod server;
mod shutdown;
mod socket;

use futures::future::join_all;
use server::handle_server;
use shutdown::setup_shutdown_handler;
use socket::{SocketError, handle_socket};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    time::Duration,
};
use tokio::time::timeout;
use tracing::{Instrument, info_span};

use crate::application::server::ServerError;

#[tracing::instrument(name = "daemon")]
pub async fn run() -> Result<(), Failure> {
    tracing::info!("starting up");

    let mut shutdown_signal = setup_shutdown_handler().map_err(Failure::Signal)?;

    let socket_signal = shutdown_signal.clone();
    let mut socket_task = tokio::spawn(
        async move { handle_socket(socket_signal).await.map_err(Failure::Socket) }
            .instrument(info_span!("socket")),
    );

    let server_signal = shutdown_signal.clone();
    let mut server_task = tokio::spawn(
        async move { handle_server(server_signal).await.map_err(Failure::Server) }
            .instrument(info_span!("server")),
    );

    tokio::select! {
        biased;

         _ = shutdown_signal.changed() => {
            tracing::info!("shutdown signal received ");
            }

        output = &mut socket_task => {
            tracing::error!("socket handler crashed: {output:?}");
            return Err(Failure::Task);
        },

        output = &mut server_task => {
            tracing::error!("server handler crashed: {output:?}");
            return Err(Failure::Task);
        }
    }

    match timeout(
        Duration::from_secs(5),
        join_all(vec![socket_task, server_task]),
    )
    .await
    {
        Ok(_) => println!("all tasks completed shutdown"),
        Err(_) => eprintln!("tasks failed to complete within shutdown timeout"),
    }

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    Server(ServerError),
    Signal(io::Error),
    Socket(SocketError),
    Task,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Server(e) => write!(f, "server error: {e}"),
            Self::Signal(io_error) => write!(f, "failed to set up shutdown listener: {io_error}"),
            Self::Socket(e) => write!(f, "socket error: {e}"),
            Self::Task => write!(f, "task crashed"),
        }
    }
}

impl Error for Failure {}
