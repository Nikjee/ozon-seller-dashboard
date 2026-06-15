use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OzonConfig {
    pub ozon_client_id: String,
    pub ozon_api_key: String,
}

impl OzonConfig {
    pub fn load() -> Result<Self, String> {
        let candidates = vec![
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.join("config.json"))),
            Some(PathBuf::from("config.json")),
        ];

        for path in candidates.into_iter().flatten() {
            if path.exists() {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
                return serde_json::from_str(&content)
                    .map_err(|e| format!("Invalid config.json: {}", e));
            }
        }

        Err("config.json not found next to executable or in current directory".into())
    }

    pub fn save(client_id: &str, api_key: &str) -> Result<(), String> {
        let config = OzonConfig {
            ozon_client_id: client_id.to_string(),
            ozon_api_key: api_key.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Serialization error: {}", e))?;

        let path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("config.json")))
            .unwrap_or_else(|| PathBuf::from("config.json"));

        std::fs::write(&path, &json)
            .map_err(|e| format!("Cannot write config.json: {}", e))?;
        Ok(())
    }
}
