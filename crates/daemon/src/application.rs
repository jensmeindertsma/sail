use core::fmt::{self, Display, Formatter};
use std::{error::Error, time::Duration};
use tokio::time::sleep;

pub async fn run() -> Result<(), ApplicationError> {
    loop {
        sleep(Duration::from_secs(10)).await;
        tracing::info!("another 10 seconds have passed");
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "application error")
    }
}

impl Error for ApplicationError {}
