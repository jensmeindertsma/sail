use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    Greeting,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    Welcome,
}
