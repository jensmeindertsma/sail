mod application;

use std::process::{ExitCode, Termination};

use tokio::runtime::Builder;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .init();

    match Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
    {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("daemon crashed: {error}");
            ExitCode::FAILURE
        }
    }
}
