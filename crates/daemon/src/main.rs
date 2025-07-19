mod application;

use tokio::runtime::Builder;

fn main() {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
}
