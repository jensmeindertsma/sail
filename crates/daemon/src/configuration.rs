use sail_core::configuration::Application;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn load() -> Self {
        // TODO: read this from the filesystem.

        Self {
            settings: Mutex::new(Settings {
                applications: Vec::new(),
                server_port: 4250,
                dashboard: DashboardSettings {
                    hostname: "dashboard.jensmeindertsma.com".to_owned(),
                },
                registry: RegistrySettings {
                    hostname: "registry.jensmeindertsma.com".to_owned(),
                },
            }),
        }
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;

        // TODO: sync this to the filesystem.
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub applications: Vec<Application>,
    pub server_port: u16,
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
