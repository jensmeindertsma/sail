use std::net::SocketAddrV4;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub hostname: String,
    pub address: SocketAddrV4,
}
