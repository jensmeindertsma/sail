mod application;

use std::process::{ExitCode, Termination};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let output = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::start());

    if let Err(error) = output {
        tracing::error!("application crashed: {error}");

        ExitCode::FAILURE
    } else {
        tracing::warn!("application has stopped");

        ExitCode::SUCCESS
    }
}

// mod shutdown {
//     use tokio::{
//         signal::unix::{SignalKind, signal},
//         sync::watch::{Receiver, channel},
//     };

//     pub type ShutdownSignal = Receiver<Signal>;

//     pub fn setup_shutdown_handler() -> ShutdownSignal {
//         let (sender, receiver) = channel(Signal);

//         tracing::debug!("set up channel for broadcasting shutdown signal");

//         tokio::spawn(async move {
//             let mut termination = signal(SignalKind::terminate())
//                 .expect("signal listener creation should succeed");

//             tracing::debug!("waiting for SIGTERM signal");

//             // Block until SIGTERM is received
//             termination.recv().await;

//             tracing::debug!("broadcasting shutdown signal");

//             sender
//                 .send(Signal)
//                 .expect("sender should be able to send value");
//         });

//         receiver
//     }

//     struct Signal;
// }

// mod socket {
//     use super::shutdown::ShutdownSignal;

//     pub async fn handle_socket(shutdown_signal: ShutdownSignal) -> Result<(), SocketError> {
//         let socket = Socket::attach();
//     }

//     #[derive(Debug)]
//     pub enum SocketError {}
// }
