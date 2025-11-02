mod application;

use std::process::{ExitCode, Termination};

use tokio::runtime::Builder;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .init();

    if let Err(error) = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(application::run())
    {
        tracing::error!("daemon crashed: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
