use crate::{
    configuration::Configuration,
    handlers::ServerHandler,
    server::{Server, ServerListenError},
    shutdown::ShutdownSignal,
};
use core::fmt::{self, Formatter};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto::Builder, graceful::GracefulShutdown},
    service::TowerToHyperService,
};
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use tracing::{error, info, info_span, Instrument};

pub async fn start_server_handler(
    configuration: Arc<Configuration>,
    mut shutdown_signal: Receiver<ShutdownSignal>,
) -> Result<GracefulShutdown, StartupFailure> {
    let mut server = match Server::listen(configuration.clone()).await {
        Ok(server) => server,
        Err(error) => {
            error!("server failed to start listening: {error}");
            return Err(StartupFailure::ListenError(error));
        }
    };

    let http = Builder::new(TokioExecutor::new());
    let graceful_shutdown = GracefulShutdown::new();

    loop {
        tokio::select! {
            _ = shutdown_signal.changed() => {
                info!("received shutdown signal");
                break
            }

            result = server.accept() => match result {
                Ok(connection) => {
                    info!("accepted new connection from {:?}", connection.address);

                    let handler = ServerHandler::new();

                    let io = TokioIo::new(connection.stream);
                    let service = TowerToHyperService::new(handler);

                    let connection = http.serve_connection(
                        io,
                        service,
                    ).into_owned();

                    let future = graceful_shutdown.watch(connection);

                    tokio::spawn(future.instrument(info_span!("handler")));

                }
                Err(error) => {
                    error!("failed to accept new connection: {error:?}");
                    continue;
                }
            }
        };
    }

    Ok(graceful_shutdown)
}

#[derive(Debug)]
pub enum StartupFailure {
    ListenError(ServerListenError),
}

impl fmt::Display for StartupFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListenError(server_listen_failure) => {
                write!(f, "failed to start server: {server_listen_failure:?}")
            }
        }
    }
}
