use crate::config::{get_config_filepath, write_api_key};
use std::io;

pub fn set_api_key_prompt() {
    println!("Input your OpenAI API Key.");
    print!("> ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let api_key = buf.to_string().trim().to_string();
    write_api_key(api_key).unwrap();

    println!(
        "Save API Key to {}.",
        get_config_filepath().to_str().unwrap()
    );
}
