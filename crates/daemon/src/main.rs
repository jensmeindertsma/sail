mod application;

use tokio::runtime::Builder;

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .init();

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
}
