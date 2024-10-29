use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto::Builder, graceful::GracefulShutdown},
    service::TowerToHyperService,
};
use std::convert::Infallible;
use tokio::{
    io,
    net::{TcpListener, ToSocketAddrs},
    sync::watch::Receiver,
};
use tower::Service;
use tracing::{error, info, info_span, Instrument};

use crate::shutdown::ShutdownSignal;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind(address: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(address).await?;

        Ok(Self { listener })
    }

    pub async fn serve_connections<S>(
        &self,
        service: S,
        mut shutdown_signal: Receiver<ShutdownSignal>,
    ) -> GracefulShutdown
    where
        S: Service<Request<Incoming>, Response = Response<Full<Bytes>>, Error = Infallible>,
        S: Clone,
        S: Send + 'static,
        S::Future: Send + 'static,
    {
        let http = Builder::new(TokioExecutor::new());
        let watcher = GracefulShutdown::new();

        loop {
            tokio::select! {
                biased;

                _ = shutdown_signal.changed() => break watcher,

                accept_result = self.listener.accept() => {
                    let (stream, origin) = match accept_result {
                        Ok(connection) => connection,
                        Err(error) => {
                            error!("failed to accept new connection: {error}");
                            continue;
                        }
                    };

                    info!("accepted new connection from {origin}");

                    let io = TokioIo::new(stream);
                    let service = TowerToHyperService::new(service.clone());

                    let connection = http.serve_connection(
                        io,
                        service,
                    ).into_owned();

                    let future = watcher.watch(connection);

                    tokio::spawn(future.instrument(info_span!("handler")));
                }
            }
        }
    }
}
