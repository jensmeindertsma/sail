use core::fmt::{self, Formatter};
use std::{error::Error, sync::Mutex};

pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub async fn load() -> Result<Self, LoadError> {
        //TODO ("load configuration from filesystem")
        //toml::from_str(s);

        Ok(Self {
            settings: Mutex::new(Settings {
                greeting: "Hello, World!".to_owned(),
                dashboard: DashboardSettings {
                    hostname: "dashboard.haven.com".to_owned(),
                },
                registry: RegistrySettings {
                    hostname: "registry.haven.com".to_owned(),
                },
            }),
        })
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;

        self.save()
    }

    fn save(&self) {
        //TODO ("save configuration to filesystem")

        //toml::from_str(s)
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub greeting: String,
    pub dashboard: DashboardSettings,
    pub registry: RegistrySettings,
}

#[derive(Clone, Debug)]
pub struct DashboardSettings {
    pub hostname: String,
}

#[derive(Clone, Debug)]
pub struct RegistrySettings {
    pub hostname: String,
}

#[derive(Debug)]
pub enum LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for LoadError {}
