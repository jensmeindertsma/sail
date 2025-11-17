mod shutdown;
mod socket;
mod state;

use arc_swap::ArcSwap;
use shutdown::setup_shutdown_handler;
use socket::handle_socket;
use state::State;
use std::{sync::Arc, time::Duration};
use tokio::{join, time::timeout};
use tracing::{Instrument, info_span};

pub async fn start() -> Result<(), Failure> {
    tracing::info!("starting up");

    let mut signal = setup_shutdown_handler().map_err(|_| Failure::Signal)?;

    let state = Arc::new(ArcSwap::new(Arc::new(
        // If we fail to load the state, we fall back to "default" state and save to file
        // on the next state modification.
        State::load().instrument(info_span!("startup")).await,
    )));

    let mut socket_task =
        tokio::spawn(handle_socket(signal.clone(), state.clone()).instrument(info_span!("socket")));

    tokio::select! {
        biased;

         _ = signal.changed() => {
            tracing::info!("received shutdown signal");
        }

        _ = &mut socket_task => {}

    }

    tracing::info!("shutting down tasks");

    let grace_period = Duration::from_secs(5);
    let (socket_join, ..) = join!(timeout(grace_period, &mut socket_task));

    if socket_join.is_err() {
        tracing::warn!("socket task did not shutdown within the grace period");
    }

    tracing::info!("shutdown complete");

    Ok(())
}

pub enum Failure {
    Signal,
}
