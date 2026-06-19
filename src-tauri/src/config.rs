use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OzonConfig {
    pub ozon_client_id: String,
    pub ozon_api_key: String,
}

fn default_path() -> PathBuf {
    dirs_config_dir().unwrap_or_else(|| PathBuf::from("config.json"))
}

fn dirs_config_dir() -> Option<PathBuf> {
    if cfg!(target_os = "macos") {
        std::env::var("HOME").ok().map(|h| {
            PathBuf::from(h)
                .join("Library/Application Support")
                .join("ozon-dashboard")
        })
    } else if cfg!(target_os = "linux") {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| PathBuf::from(h).join(".config"))
            })
            .map(|p| p.join("ozon-dashboard"))
    } else {
        None
    }
}

impl OzonConfig {
    pub fn load() -> Result<Self, String> {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(d) = dirs_config_dir() {
            candidates.push(d.join("config.json"));
        }
        candidates.push(
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.join("config.json")))
                .unwrap_or_else(|| PathBuf::from("config.json")),
        );
        candidates.push(PathBuf::from("config.json"));

        for path in &candidates {
            if path.exists() {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
                return serde_json::from_str(&content)
                    .map_err(|e| format!("Invalid config.json: {}", e));
            }
        }

        Err("config.json not found".into())
    }

    pub fn save(client_id: &str, api_key: &str) -> Result<(), String> {
        Self::save_to(&default_path(), client_id, api_key)
    }

    pub fn save_to(path: &Path, client_id: &str, api_key: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create config dir: {}", e))?;
        }
        let config = OzonConfig {
            ozon_client_id: client_id.to_string(),
            ozon_api_key: api_key.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Serialization error: {}", e))?;
        std::fs::write(path, &json)
            .map_err(|e| format!("Cannot write config.json: {}", e))?;
        Ok(())
    }
}
