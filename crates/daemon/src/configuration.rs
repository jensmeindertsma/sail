use std::{fs, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    pub fn load() -> Result<Self, toml::de::Error> {
        match fs::read_to_string("/etc/sail/configuration.toml") {
            Ok(contents) => toml::from_str(&contents).map(|settings| Self {
                settings: Mutex::new(settings),
            }),
            Err(_) => Ok(Self {
                settings: Mutex::new(Settings::default()),
            }),
        }
    }

    pub fn get(&self) -> Settings {
        self.settings
            .lock()
            // TODO: handle this error case with tracing
            .expect("settings lock should not become poisoned")
            .clone()
    }

    pub fn set(&self, new_settings: Settings) {
        *self
            .settings
            .lock()
            // TODO: handle this error case with tracing
            .expect("settings lock should not become poisoned") = new_settings;

        self.save()
    }

    pub fn save(&self) {
        let _ = fs::create_dir_all("/etc/sail");
        let _ = fs::write(
            "/etc/sail/configuration.toml",
            toml::to_string_pretty(
                &self
                    .settings
                    .lock()
                    .expect("settings lock should not become poisoned")
                    .clone(),
            )
            .expect("serialization should not fail"),
        );
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub port: u16,
    pub greeting: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 4250,
            greeting: "Greetings from Sail".to_owned(),
        }
    }
}
