use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{self, Receiver},
};

pub struct ShutdownSignal;

pub fn setup_shutdown_listener() -> Receiver<ShutdownSignal> {
    let (sender, receiver) = watch::channel(ShutdownSignal);

    let termination_sender = sender.clone();

    tokio::spawn(async move {
        let mut termination = signal(SignalKind::terminate()).unwrap();

        // This tasks blocks until the future on the next line resolves,
        // which happens when a SIGTERM is received.
        termination.recv().await;

        // Then this line will run which initiates the shutdown process
        // in all receivers.
        termination_sender.send(ShutdownSignal).unwrap();
    });

    receiver
}
