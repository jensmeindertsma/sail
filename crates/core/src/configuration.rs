use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;
use std::{fs, sync::Mutex};
use tracing::error;

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
    pub token: String,
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

#[derive(Debug)]
pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn load() -> Self {
        let settings = match fs::read_to_string("/etc/sail/configuration.toml") {
            Ok(contents) => toml::from_str::<Settings>(&contents)
                .expect("deserialization of settings should not fail"),
            Err(_error) => {
                error!("no configuration file found, using defaults");
                Settings::default()
            }
        };

        let self_ = Self {
            settings: Mutex::new(settings),
        };

        self_.save();

        self_
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;

        self.save()
    }

    fn save(&self) {
        let serialized = toml::to_string_pretty(&self.get())
            .expect("serialization of settings should never fail");

        if !fs::exists("/etc/sail").expect("existance checker should not fail") {
            fs::create_dir("/etc/sail").expect("creating directory should not fail")
        }

        fs::write("/etc/sail/configuration.toml", serialized)
            .expect("writing configuration to disk should not fail");
    }
}
