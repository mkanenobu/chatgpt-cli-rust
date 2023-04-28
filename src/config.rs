use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub system_context: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

impl Config {
    pub fn new() -> Result<Config> {
        read_config_by_json_file()
    }

    pub fn default() -> Config {
        Config {
            api_key: None,
            system_context: None,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.7),
            top_p: Some(1.0),
        }
    }
}

fn read_config_by_json_file() -> Result<Config> {
    let path = get_config_filepath();
    let config_string = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_string)?;
    Ok(config)
}

// ~/.config/chatgpt-repl/config.toml
pub fn get_config_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("chatgpt-repl");
    path.push("config.toml");
    path
}

fn write_config(config: Config) -> Result<()> {
    let path = get_config_filepath();
    let config_string = toml::to_string_pretty(&config)?;

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, config_string)?;
    Ok(())
}

pub fn write_api_key(api_key: String) -> Result<()> {
    let mut config = Config::new().unwrap_or(Config::default());
    config.api_key = Some(api_key);
    write_config(config)
}
