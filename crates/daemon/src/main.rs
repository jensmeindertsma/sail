mod application;

use std::process::{ExitCode, Termination};
use tokio::runtime::Builder;
use tracing::info_span;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_level(true)
        .init();

    info_span!("daemon").in_scope(|| {
        match Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(application::run())
        {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => {
                tracing::error!("{failure}");
                ExitCode::FAILURE
            }
        }
    })
}
