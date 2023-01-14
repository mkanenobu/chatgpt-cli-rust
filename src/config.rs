use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

impl Config {
    pub fn new() -> Config {
        read_config_by_json_file()
    }

    pub fn init(&self) {
        // async-openai can only use env value
        env::set_var("OPENAI_API_KEY", &self.api_key);
    }
}

fn read_config_by_json_file() -> Config {
    let path = config_filepath();
    let config_string = fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&config_string).unwrap();
    config
}

fn config_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("chatgpt-cli");
    path.push("config.json");
    path
}
