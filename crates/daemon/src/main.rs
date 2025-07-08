mod application;

use std::process::{ExitCode, Termination};
use tokio::runtime::Builder;
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    if let Err(error) = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
    {
        tracing::error!("application crashed: {error}");

        ExitCode::FAILURE
    } else {
        tracing::info!("application stopped");

        ExitCode::SUCCESS
    }
}
