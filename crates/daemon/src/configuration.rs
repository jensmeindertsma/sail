use sail_core::configuration::Application;
use std::sync::Mutex;

pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn load() -> Self {
        Self {
            settings: Mutex::new(Settings {
                applications: Vec::new(),
            }),
        }
    }

    pub fn get(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self.settings.lock().unwrap() = new_settings;
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub applications: Vec<Application>,
}
