use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{Receiver, channel},
};
use tracing::{Instrument, debug_span, instrument};

#[instrument(name = "setup")]
pub fn setup_shutdown_handler() -> Result<Receiver<()>, ()> {
    let (sender, receiver) = channel(());

    let mut signal = signal(SignalKind::terminate()).map_err(|_| {
        tracing::error!("failed to create signal listener");
    })?;

    tokio::spawn(
        async move {
            signal.recv().await;

            tracing::debug!("broadcasting shutdown");

            sender.send(())
        }
        .instrument(debug_span!("signal")),
    );

    Ok(receiver)
}
