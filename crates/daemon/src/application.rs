use tokio::time::{self, Duration};

pub async fn run() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .init();

    tracing::info!("starting up");

    loop {
        time::sleep(Duration::from_secs(10)).await;
        tracing::info!("running")
    }
}
