use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        read_config_by_json_file(config_filepath()).unwrap_or(Config::default())
    }

    fn default() -> Config {
        Config { api_key: None }
    }
}

fn read_config_by_json_file(path: PathBuf) -> Result<Config> {
    let config_string = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_string)?;
    Ok(config)
}

// ~/.config/chatgpt-repl/config.json
pub fn config_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("chatgpt-repl");
    path.push("config.json");
    path
}

pub fn write_config(config: Config) -> Result<()> {
    let path = config_filepath();
    fs::create_dir_all(path.parent().unwrap())?;
    let config_string = serde_json::to_string_pretty(&config)?;
    fs::write(path, config_string)?;
    Ok(())
}
