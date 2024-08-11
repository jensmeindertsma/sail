use crate::app::server::interface::InterfaceSettings;
use crate::app::server::registry::RegistrySettings;
use std::{collections::HashMap, net::SocketAddrV4, sync::Mutex};

pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn from_filesystem() -> Result<Self, ConfigurationError> {
        let mut applications = HashMap::new();

        applications.insert(
            "helloworld".to_owned(),
            Application {
                address: SocketAddrV4::new("127.0.0.1".parse().unwrap(), 4201),
            },
        );

        Ok(Self {
            settings: Mutex::new(Settings {
                applications,
                interface: InterfaceSettings {
                    hostname: "sail.jensmeindertsma.com".to_owned(),
                },
                registry: RegistrySettings {
                    hostname: "registry.jensmeindertsma.com".to_owned(),
                },
            }),
        })
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    // pub fn set(&self, new_settings: Settings) {
    //     *self.settings.lock().unwrap() = new_settings;
    // }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub applications: HashMap<String, Application>,
    pub interface: InterfaceSettings,
    pub registry: RegistrySettings,
}

#[derive(Clone, Debug)]
pub struct Application {
    pub address: SocketAddrV4,
}

#[derive(Debug)]
pub enum ConfigurationError {}
