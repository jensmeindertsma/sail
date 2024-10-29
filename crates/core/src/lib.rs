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
    pub listening_on: SocketAddr,
    pub dashboard_hostname: String,
    pub registry_hostname: String,
}
