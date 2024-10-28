use core::fmt::{self, Formatter};
use std::error::Error;

pub fn configure(setting: &str, value: &str) -> Result<(), ConfigureError> {
    println!("TODO: configure {setting} to {value}");

    Ok(())
}

#[derive(Debug)]
pub enum ConfigureError {}

impl fmt::Display for ConfigureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo!")
    }
}

impl Error for ConfigureError {}
