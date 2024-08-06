use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,

    #[serde(rename = "systemContext")]
    pub system_context: Option<String>,

    #[serde(rename = "model")]
    pub model: Option<String>,

    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,

    #[serde(rename = "topP")]
    pub top_p: Option<f32>,

    #[serde(rename = "usePerplexityApi")]
    pub use_perplexity_api: Option<bool>,
}

impl Config {
    pub fn new() -> Result<Config> {
        read_config_by_json_file()
    }

    pub fn default() -> Config {
        Config {
            api_key: None,
            system_context: None,
            model: Some("gpt-4".to_string()),
            temperature: Some(0.7),
            top_p: Some(1.0),
            use_perplexity_api: Some(false),
        }
    }
}

fn read_config_by_json_file() -> Result<Config> {
    let path = get_config_filepath();
    let config_string = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_string)?;
    Ok(config)
}

pub fn get_home_path() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap())
}

// ~/.config/chatgpt-repl/config.json
pub fn get_config_filepath() -> PathBuf {
    let mut path = get_home_path();
    path.push(".config");
    path.push("chatgpt-repl");
    path.push("config.json");
    path
}

fn write_config(config: Config) -> Result<()> {
    let path = get_config_filepath();
    let config_string = serde_json::to_string_pretty(&config)?;

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, config_string)?;
    Ok(())
}

pub fn write_api_key(api_key: String) -> Result<()> {
    let mut config = Config::new().unwrap_or(Config::default());
    config.api_key = Some(api_key);
    write_config(config)
}
