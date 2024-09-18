use tokio::{
    signal::unix::{signal, SignalKind},
    sync::watch::{self, Receiver, Sender},
};

#[derive(Debug)]
pub struct ShutdownSignal;

pub fn setup_shutdown_listener() -> (Receiver<ShutdownSignal>, Sender<ShutdownSignal>) {
    let (sender, receiver) = watch::channel(ShutdownSignal);

    let termination_sender = sender.clone();
    tokio::spawn(async move {
        let mut termination = signal(SignalKind::terminate()).unwrap();

        termination.recv().await;
        termination_sender.send(ShutdownSignal).unwrap();
    });

    (receiver, sender)
}
