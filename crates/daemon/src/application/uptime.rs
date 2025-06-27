use tokio::{
    select,
    time::{Duration, Instant, sleep},
};

use crate::application::shutdown::ShutdownSignal;

pub async fn log_uptime(mut shutdown_signal: ShutdownSignal) {
    let start = Instant::now();

    loop {
        let shutdown = shutdown_signal.receive();
        let timeout = sleep(Duration::from_secs(10));

        select! {
            biased;

            _ = shutdown => {
                tracing::info!("shutting down")
            },

            _ = timeout => {
                tracing::info!("up for {} seconds", start.elapsed().as_secs())
            }
        }
    }
}
