pub mod exit;

use std::io;
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch::{Receiver, channel},
};

pub fn setup_shutdown_handler() -> io::Result<ShutdownSignal> {
    let (sender, receiver) = channel(());

    let mut signal = signal(SignalKind::terminate())?;

    tokio::spawn(async move {
        signal.recv().await;

        sender.send(())
    });

    Ok(receiver)
}

pub type ShutdownSignal = Receiver<()>;
