mod connection;

use super::shutdown::ShutdownSignal;
use connection::handle_connection;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};
use tokio::net::TcpListener;
use tracing::{Instrument, info_span};

pub async fn handle_server(mut shutdown_signal: ShutdownSignal) -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:1312")
        .await
        .map_err(ServerError::CreateListener)?;

    tracing::info!("creating listener");

    loop {
        tokio::select! {
            biased;

            _ = shutdown_signal.changed() => {
                return Ok(())
            }

            result = listener.accept() => {
                match result {
                    Ok((stream, address)) => {
                        tracing::info!("handling new connection from {address:?}");
                        tokio::spawn(handle_connection(stream).instrument(info_span!("handler")));


                    },
                    Err(error) => {
                        return Err(ServerError::Accept(error))
                    },
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ServerError {
    Accept(io::Error),
    CreateListener(io::Error),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept(io_error) => write!(f, "failed to accept socket connection: {io_error}"),
            Self::CreateListener(io_error) => write!(f, "failed to create listener: {io_error}"),
        }
    }
}

impl Error for ServerError {}
