use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{Receiver, channel},
};

pub struct ShutdownSignal;

pub fn setup_shutdown_handler() -> Receiver<ShutdownSignal> {
    let (sender, receiver) = channel(ShutdownSignal);

    let termination_sender = sender.clone();

    tokio::spawn(async move {
        let mut termination = signal(SignalKind::terminate()).expect("this should never fail");

        // This tasks blocks until the future resolves,
        // which happens when a SIGTERM is received.
        termination.recv().await;

        // Then this line will run which initiates the shutdown process
        // in all receivers.
        termination_sender
            .send(ShutdownSignal)
            .expect("should this fail then panicking is already the only option");
    });

    receiver
}
