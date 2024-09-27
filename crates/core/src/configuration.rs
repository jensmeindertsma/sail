use std::net::SocketAddrV4;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub applications: Vec<Application>,
    pub server_port: u16,
    pub dashboard: DashboardSettings,
    pub registry: RegistrySettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    pub hostname: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrySettings {
    pub hostname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub hostname: String,
    // Sail will show a placeholder page if this is not set. This property is typically
    // automatically set by Sail
    pub address: Option<SocketAddrV4>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            applications: Vec::new(),
            server_port: 4250,
            dashboard: DashboardSettings {
                hostname: String::from("sail.jensmeindertsma.com"),
            },
            registry: RegistrySettings {
                hostname: String::from("registry.jensmeindertsma.com"),
            },
        }
    }
}
