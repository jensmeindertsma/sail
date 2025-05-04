use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    Greet { message: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    Greeting { message: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketError {
    Connect,
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unknown socket error!")
    }
}

impl Error for SocketError {}
