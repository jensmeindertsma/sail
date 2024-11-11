use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tracing::error;

pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn load() -> Self {
        let setting = match fs::read_to_string("/etc/sail/configuration.toml")
            .map(|s| toml::from_str::<Settings>(&s))
        {
            Ok(Ok(settings)) => settings,
            _default => Settings {
                greeting: "Hello, World!".to_owned(),

                registry: RegistrySettings {
                    hostname: "registry.jensmeindertsma.com".to_owned(),
                },
            },
        };

        let configuration = Self {
            settings: Mutex::new(setting),
        };

        configuration.save();

        configuration
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;

        self.save()
    }

    fn save(&self) {
        let str = match toml::to_string_pretty(&self.settings) {
            Ok(str) => str,
            Err(e) => {
                error!("failed to serialize settings: {e}");
                return;
            }
        };

        if let Err(e) = fs::write("/etc/sail/configuration.toml", str) {
            error!("failed to write configuration to disk {e}")
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub greeting: String,
    pub registry: RegistrySettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrySettings {
    pub hostname: String,
}
