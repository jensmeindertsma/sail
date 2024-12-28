mod configuration;
mod server;
mod shutdown;
mod socket;

use configuration::Configuration;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto::Builder, graceful::GracefulShutdown as ConnectionWatcher},
    service::TowerToHyperService,
};
use server::{ServerHandler, ServerListener};
use shutdown::setup_shutdown_listener;
use socket::{SocketConnector, SocketHandler, SocketListener};
use std::{process::ExitCode, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{Instrument, Level, info_span};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let mut shutdown_listener = setup_shutdown_listener();
    let configuration = Arc::new(Configuration::load().unwrap());

    tracing::info!("loaded configuration: {:#?}", configuration.get());

    let socket_cfg = configuration.clone();
    let socket_task = tokio::spawn(
        async move {
            let listener = SocketListener::attach().unwrap();

            loop {
                let stream = listener.accept().await.unwrap();

                tracing::info!("accepted new socket connection");

                // This one handles `SocketRequest`s, so we'll need an
                // intermediate connection handler to act between connection
                // and request. We do that below with SocketConnector
                let service = SocketHandler::new(socket_cfg.clone());

                tokio::spawn(
                    async move { SocketConnector::new(stream).serve_connection(service).await }
                        .instrument(info_span!("handler")),
                );
            }
        }
        .instrument(info_span!("socket")),
    );

    let connection_watcher = ConnectionWatcher::new();

    let server_cfg = configuration.clone();
    let server_task = tokio::spawn(
        async move {
            let mut listener = ServerListener::bind(server_cfg.get().port).await.unwrap();

            loop {
                let stream = listener.accept().await.unwrap();

                tracing::info!("accepted new server connection");

                // We create one `Service` per connection, which will handle
                // all requests for that connection.
                let service = ServerHandler::new(server_cfg.clone());

                tokio::spawn(
                    async move {
                        let builder = Builder::new(TokioExecutor::new());

                        builder
                            .serve_connection(
                                TokioIo::new(stream),
                                TowerToHyperService::new(service),
                            )
                            .await
                    }
                    .instrument(info_span!("handler")),
                );
            }
        }
        .instrument(info_span!("server")),
    );

    let mut crashed = false;

    tokio::select! {
        biased;

        _ = socket_task => {
            tracing::error!("socket handler terminated unexpectedly!");
            crashed = true;
        }

        // When we receive the shutdown signal we break from the `select!`
        // and stop polling the futures.
        _ = shutdown_listener.changed() => {

        }

        // Only if we have no shutdown signal AND the socket handler is still active
        // do we process requests.
        //
        // NOTE: every `poll` of `server_task` will result in one new connection
        // being accepted, which is then immediately spawned into its own task,
        // so the server loop continues, hits the next `accept().await` and yields.
        _ = server_task => {
            tracing::error!("server handler terminated unexpectedly");
            crashed = true;
        }
    };

    // As soon as we leave the main `select!` phase we should attempt to
    // gracefully terminate existing connections.
    tokio::select! {
        _ = connection_watcher.shutdown() => {
            tracing::info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            tracing::error!("timed out wait for all connections to close");
        }
    };

    if crashed {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
