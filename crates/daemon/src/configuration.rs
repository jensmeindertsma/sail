use core::panic;
use std::{fs, sync::Mutex};

use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug)]
pub struct Configuration {
    settings: Mutex<Settings>,
}

impl Configuration {
    #[instrument(name = "configuration", skip_all)]
    pub fn load() -> Result<Self, toml::de::Error> {
        tracing::debug!("loading configuration from file");

        match fs::read_to_string("/etc/sail/configuration.toml") {
            Ok(contents) => toml::from_str(&contents).map(|settings| Self {
                settings: Mutex::new(settings),
            }),
            Err(_) => Ok(Self {
                settings: Mutex::new(Settings::default()),
            }),
        }
    }

    #[instrument(name = "configuration", skip_all)]
    pub fn get(&self) -> Settings {
        match self.settings.lock() {
            Ok(settings) => settings.clone(),
            Err(_) => {
                tracing::error!("configuration settings lock is poisoned, panicking");
                panic!("configuration settings lock has been poisoned")
            }
        }
    }

    #[instrument(name = "configuration", skip_all)]
    pub fn set(&self, new_settings: Settings) {
        let mut current = match self.settings.lock() {
            Ok(settings) => settings,
            Err(_) => {
                tracing::error!("configuration settings lock is poisoned, panicking");
                panic!("configuration settings lock has been poisoned");
            }
        };

        tracing::debug!("locked settings for modification");

        *current = new_settings;

        tracing::debug!("modified settings");

        drop(current);

        self.save()
    }

    #[instrument(name = "configuration", skip_all)]
    pub fn save(&self) {
        tracing::debug!("saving settings");

        let _ = fs::create_dir_all("/etc/sail");

        tracing::debug!("created /etc/sail directory");

        let settings = self.get();

        tracing::debug!("got settings for saving");

        if let Err(io_error) = fs::write(
            "/etc/sail/configuration.toml",
            match toml::to_string_pretty(&settings) {
                Ok(string) => string,
                Err(error) => {
                    tracing::error!("failed to serialize settings for saving: {error}");
                    return;
                }
            },
        ) {
            tracing::error!(
                "failed to write settings to `/etc/sail/configuration.toml`: {io_error}"
            )
        }

        tracing::debug!("saved settings to file");
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
