use tokio::time::{self, Duration};
use tracing::instrument;

#[instrument(name = "daemon")]
pub async fn run() {
    tracing::info!("starting up");

    loop {
        time::sleep(Duration::from_secs(10)).await;
        tracing::info!("running");
    }
}
