mod socket;

use tokio::runtime::Builder;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
}

mod application {
    pub async fn run() {
        loop {}
    }
}
