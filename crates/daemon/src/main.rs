mod app;
mod configuration;
mod server;
mod socket;

use app::{ServerHandler, SocketHandler};
use configuration::Configuration;
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto::Builder as ConnectionBuilder, graceful::GracefulShutdown},
    service::TowerToHyperService,
};
use sail_core::socket::{SocketRequest, SocketResponse};
use server::Server;
use socket::{Socket, SocketError};
use std::{process::ExitCode, sync::Arc, time::Duration};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::watch,
    task::JoinHandle,
    time::sleep,
};
use tracing::{error, info, span, Instrument, Level};

// This is the entrypoint for the Sail daemon.
// It does 4 things:
// 1. Starts up a socket connection handler that processes requests coming from the CLI.
// 2. Starts up a server connection handler that processes HTTP requests coming from Nginx.
// 3. Gracefully terminates in-progress connections when a shutdown signal is received.
// 4. Waits for the socket handler to quit when a shutdown signal is received.
#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    let shutdown_listener = create_shutdown_listener();

    let configuration = Arc::new(match Configuration::from_filesystem() {
        Ok(configuration) => configuration,
    });

    // PART 1
    // We start a handling loop (on a separate thread) that handles incoming
    // socket connections and processes their requests.

    let mut shutdown_socket = shutdown_listener.clone();
    let socket_task_handle: JoinHandle<Result<(), SocketError>> = tokio::spawn(
        async move {
            let socket = Socket::attach().map_err(|error| {
                error!("failed to connect to socket: {error:?}");
                error
            })?;

            info!("attached to systemd socket");

            loop {
                tokio::select! {
                    _ = shutdown_socket.changed() => {
                        info!("received termination signal, quitting!");
                        break
                    },

                    result = socket.accept() => {
                        let connection = match result {
                            Ok(c) => c,
                            Err(e) => {
                                error!("failed to accept socket connection: {e:?}");
                                continue;
                            }
                        };

                        let service = SocketHandler::new();
                        let future = service.handle_connection(connection);

                        tokio::spawn(future.instrument(span!(Level::INFO, "handler")));
                    }
                }
            }

            Ok(())
        }
        .instrument(span!(Level::INFO, "socket")),
    );

    // PART 2
    // Here we set up a server connection handling loop

    let span = span!(Level::INFO, "server").entered();

    let mut shutdown_server = shutdown_listener;

    let server = match Server::new().await {
        Ok(server) => server,
        Err(error) => {
            error!("failed to set up server: {error:?}");
            return ExitCode::FAILURE;
        }
    };

    let http_stack = ConnectionBuilder::new(TokioExecutor::new());
    let shutdown_helper = GracefulShutdown::new();

    loop {
        tokio::select! {
            _ = shutdown_server.changed() => {
                info!("received termination signal, quitting!");
                break
            },

            result = server.accept() => {
                let connection = match result {
                    Ok(c) => c,
                    Err(e) => {
                        error!("failed to accept server connection: {e:?}");
                        continue;
                    }
                };

                info!("accepted connection from {}", connection.address);

                let handler = ServerHandler::new(configuration.clone());
                let connection = shutdown_helper.watch(
                    http_stack
                        .serve_connection_with_upgrades(
                            TokioIo::new(connection.stream),
                            TowerToHyperService::new(handler)
                        )
                        .into_owned()
                );

                tokio::spawn(
                    async move {
                        if let Err(error) = connection.await {
                            error!("failed to serve http connection: {error:?}")
                        }
                    }
                    .instrument(span!(Level::INFO, "handler")),
                );
            }
        };
    }

    // PART 3
    // When we break from the connection acceptance loop (because we received
    // a shutdown signal), we gracefully terminate the currently in-progress
    // connections.

    tokio::select! {
        _ = shutdown_helper.shutdown() => {
            info!("all connections gracefully closed");
        },
        _ = sleep(Duration::from_secs(10)) => {
            error!("timed out wait for all connections to close");
        }
    }

    span.exit();

    let span = span!(Level::INFO, "daemon").entered();

    // PART 4
    // Lastly, we wait for the socket handler to stop before we
    // end the process.

    info!("succesfully stopped the server");

    if let Err(error) = socket_task_handle.await {
        error!("failed to complete socket handler task: {error:?}")
    } else {
        info!("successfully stopped the socket handler")
    }

    span.exit();

    ExitCode::SUCCESS
}

fn create_shutdown_listener() -> watch::Receiver<()> {
    let (stop_tx, stop_rx) = watch::channel(());

    tokio::spawn(async move {
        let mut termination = signal(SignalKind::terminate()).unwrap();
        termination.recv().await;
        stop_tx.send(()).unwrap();
    });

    stop_rx
}

fn handle_socket_request(request: SocketRequest) -> SocketResponse {
    match request {
        SocketRequest::Greeting => SocketResponse::Okay,
    }
}

fn handle_server_request(request: hyper::Request<Incoming>) -> ServerResponse {}
