use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub model: String,
    pub max_tokens: u16,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let config_path = current_dir.join("config.json");
    let config_file = std::fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_file)?;
    Ok(config)
}