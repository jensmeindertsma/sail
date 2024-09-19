use crate::{
    configuration::Configuration,
    handlers::SocketHandler,
    shutdown::ShutdownSignal,
    socket::{Socket, SocketAttachmentError},
};
use core::fmt::{self, Formatter};
use std::sync::Arc;
use tokio::sync::watch::{Receiver, Sender};
use tracing::{error, info, info_span, Instrument};

pub async fn start_socket_handler(
    configuration: Arc<Configuration>,
    mut shutdown_signal: Receiver<ShutdownSignal>,
    request_shutdown: Sender<ShutdownSignal>,
) -> Result<(), StartupFailure> {
    let mut socket = match Socket::attach() {
        Ok(socket) => socket,
        Err(error) => {
            // If we fail here, we request a complete shutdown, because otherwise
            // the server will just startup and the daemon will be in a partially
            // functioning state.
            error!("socket failed to attach: {error}");

            request_shutdown
                .send(ShutdownSignal)
                .expect("requesting shutdown should not fail");

            return Err(StartupFailure::SocketAttachment(error));
        }
    };

    info!("attached to socket");

    loop {
        tokio::select! {
            _ = shutdown_signal.changed() => {
                info!("received shutdown signal");
                break
            }

            result = socket.accept() => match result {
                Ok(connection) => {
                    info!("accepted new connection from {:?}", connection.address);

                    let configuration = configuration.clone();

                    tokio::spawn(
                        async move {
                            let mut handler = SocketHandler::new(configuration);

                            handler.serve_connection(connection).await;
                        }.instrument(info_span!("handler"))
                    );
                }
                Err(error) => {
                    error!("failed to accept new connection: {error:?}");
                    continue;
                }
            }
        };
    }

    Ok(())
}

#[derive(Debug)]
pub enum StartupFailure {
    SocketAttachment(SocketAttachmentError),
}

impl fmt::Display for StartupFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SocketAttachment(error) => {
                write!(f, "failed to attach to socket: {error:?}")
            }
        }
    }
}
