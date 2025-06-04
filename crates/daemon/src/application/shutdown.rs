use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{Receiver, channel},
};

pub fn create_shutdown_listener() -> ShutdownSignal {
    tracing::info!("setting up SIGTERM handling");

    let (sender, receiver) = channel(());

    tokio::spawn(async move {
        let mut signal = signal(SignalKind::terminate()).unwrap();

        // Block the task until SIGTERM
        signal.recv().await;

        sender.send(())
    });

    ShutdownSignal { receiver }
}

#[derive(Clone, Debug)]
pub struct ShutdownSignal {
    receiver: Receiver<()>,
}

impl ShutdownSignal {
    pub async fn received(&mut self) {
        self.receiver.changed().await.unwrap()
    }
}
