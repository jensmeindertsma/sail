use sail_core::configuration::Settings;
use std::{fs, sync::Mutex};
use tracing::error;

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
