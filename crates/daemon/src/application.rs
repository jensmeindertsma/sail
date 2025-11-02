use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::{
    signal::unix::{SignalKind, signal},
    time::{self, Duration},
};
#[tracing::instrument(name = "daemon")]
pub async fn run() -> Result<(), Failure> {
    tracing::info!("starting up");

    let mut sigterm = signal(SignalKind::terminate()).map_err(|_| Failure::SignalListener)?;

    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_secs(10)) => {
                tracing::info!("running");
            }

            _ = sigterm.recv() => {
                tracing::info!("received SIGTERM, shutting down gracefully");
                break;
            }

        }
    }

    tracing::info!("cleanup done, exiting");

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    SignalListener,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignalListener => write!(f, "failed to create signal listener"),
        }
    }
}

impl Error for Failure {}
