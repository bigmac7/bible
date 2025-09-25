use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    #[serde(rename = "TRANSLATION")]
    pub translation: Option<String>,
    pub default_translation: Option<String>,
}

fn get_config_path() -> Result<PathBuf, AppError> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| AppError::NotFound("a valid config directory".to_string()))?;
    path.push("bible-cli");
    fs::create_dir_all(&path)?;
    path.push("config.json");
    Ok(path)
}

pub fn load_config() -> Result<Config, AppError> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content).map_err(|e| AppError::Io(e.into()))?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), AppError> {
    let path = get_config_path()?;
    let content = serde_json::to_string_pretty(config).map_err(|e| AppError::Io(e.into()))?;
    fs::write(path, content)?;
    Ok(())
}

pub fn get_default_translation() -> Result<String, AppError> {
    let default_from_env = env::var("BIBLE_DEFAULT_TRANSLATION").ok();
    let config = load_config()?;

    let default = default_from_env
        .or(config.translation)
        .or(config.default_translation)
        .unwrap_or_else(|| "KJV".to_string());
    Ok(default)
}
