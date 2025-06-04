mod handler;
mod setup;

use super::shutdown::ShutdownSignal;
use futures::future::join_all;
use handler::handle_connection;
use setup::create_socket;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::Duration,
};
use tokio::{select, time::sleep};
use tracing::{Instrument, info_span};

pub async fn handle_socket(mut shutdown_signal: ShutdownSignal) -> Result<(), SocketError> {
    tracing::info!("starting up socket handler");

    let socket = create_socket();

    tracing::debug!("bound to socket");

    let mut handles = Vec::new();
    let mut id = 1;

    loop {
        select! {
            biased;

            _ = shutdown_signal.received() => {
                tracing::info!("initiation socket shutdown");
                break
            }

            connection = socket.accept() => {
                let (stream, address) = connection.unwrap();

                tracing::info!("accepted new connection from {address:?}");

                let handle =  tokio::spawn(handle_connection(stream).instrument(info_span!("handler", %id)));

                handles.push(handle);
                id += 1;
            }
        }
    }

    handles.retain(|handle| !handle.is_finished());

    tracing::debug!(
        "initiating graceful shutdown for {} handlers",
        handles.len()
    );

    let shutdown = join_all(handles);
    let timeout = sleep(Duration::from_secs(2));

    tokio::select! {
        _ = shutdown => {
            tracing::debug!("all handlers completed before timeout");
        }
        _ = timeout => {
            tracing::warn!("timeout reached; some handlers may still be running");
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub enum SocketError {}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "socket error")
    }
}

impl Error for SocketError {}
