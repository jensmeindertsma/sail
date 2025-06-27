use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{Receiver, Sender, channel},
};

#[derive(Clone, Debug)]
pub struct ShutdownSignal {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        let (sender, receiver) = channel(());

        tokio::spawn({
            let sender = sender.clone();
            async move {
                let mut signal = signal(SignalKind::terminate()).unwrap();

                // Block the task until SIGTERM
                signal.recv().await;

                let _ = sender.send(());
            }
        });

        Self { sender, receiver }
    }

    pub fn broadcast(&self) {
        let _ = self.sender.send(());
    }

    pub async fn receive(&mut self) {
        self.receiver.changed().await.unwrap()
    }
}
