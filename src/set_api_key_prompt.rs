use crate::config::{config_filepath, write_config, Config};
use std::io;

pub fn set_api_key_prompt() {
    println!("Input your OpenAI API Key.");
    print!("> ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let api_key = buf.to_string().trim().to_string();
    write_config(Config {
        api_key: Some(api_key),
    })
    .unwrap();

    println!("Save API Key to {}.", config_filepath().to_str().unwrap());
}
