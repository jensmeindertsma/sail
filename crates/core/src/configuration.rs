use std::net::SocketAddrV4;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub hostname: String,
    // Sail will show a placeholder page if this is not set. This property is typically
    // automatically set by Sail
    pub address: Option<SocketAddrV4>,
}
