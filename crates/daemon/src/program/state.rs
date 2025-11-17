use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct State {
    pub count: usize,
}

impl State {
    pub async fn load() -> Self {
        match fs::read_to_string("/var/lib/sail/state.json").await {
            Err(error) => {
                tracing::error!("failed to read state file: {error}");
                Default::default()
            }
            Ok(text) => match serde_json::from_str(&text) {
                Ok(state) => state,
                Err(error) => {
                    tracing::error!("failed to deserialize state file: {error}");
                    Default::default()
                }
            },
        }
    }

    pub async fn save_to_file(&self) {
        let _ = serde_json::to_string(self)
            .and_then(|text| fs::write("/var/lib/sail/state.json", text).await)
            .await;
    }
}
