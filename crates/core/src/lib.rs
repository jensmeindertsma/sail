use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SocketRequest {
    Status,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SocketResponse {
    Other,
    Status(Status),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Status {
    listening_on: SocketAddr,
    dashboard_hostname: String,
    registry_hostname: String,
}
