use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        read_config_by_json_file(config_filepath())
    }

    pub fn init(&self) {
        // async-openai can only use env value
        env::set_var("OPENAI_API_KEY", &self.api_key);
    }
}

fn read_config_by_json_file(path: PathBuf) -> Result<Config> {
    let config_string = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_string)?;
    validate(&config)?;
    Ok(config)
}

fn config_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("chatgpt-cli");
    path.push("config.json");
    path
}

fn validate(config: &Config) -> Result<()> {
    if config.api_key.is_empty() {
        bail!("api_key is empty");
    }
    Ok(())
}
