use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    Greet { message: String },
    SetGreeting { message: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    Greeting { message: String },
    Success,
}
