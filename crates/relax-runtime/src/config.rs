use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub config_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            model: "gpt-4.1-mini".to_string(),
            config_file: PathBuf::from("relax.toml"),
        }
    }
}

impl Config {
    pub fn load_from_workspace(
        workspace: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let workspace = workspace.as_ref();
        let config_file = workspace.join("relax.toml");
        let mut config = Self {
            config_file: config_file.clone(),
            ..Self::default()
        };

        if config_file.exists() {
            let raw = fs::read_to_string(&config_file)?;
            let value: toml::Value = toml::from_str(&raw)?;

            if let Some(provider) = value.get("provider").and_then(toml::Value::as_str) {
                config.provider = provider.to_string();
            }
            if let Some(model) = value.get("model").and_then(toml::Value::as_str) {
                config.model = model.to_string();
            }
        }

        if let Ok(provider) = std::env::var("RELAX_PROVIDER") {
            config.provider = provider;
        }
        if let Ok(model) = std::env::var("RELAX_MODEL") {
            config.model = model;
        }

        Ok(config)
    }
}
