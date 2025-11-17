use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    Status,
    Add,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    Status { count: usize },
    AddComplete,
}
