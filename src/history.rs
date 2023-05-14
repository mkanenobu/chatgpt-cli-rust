use crate::config::get_home_path;
use rustyline::{history::DefaultHistory, Editor};
use std::{env, fs, path::PathBuf, process};

const FILE_NAME: &str = ".chatgpt-repl-history";

fn get_history_filepath() -> PathBuf {
    let mut path = get_home_path();
    path.push(FILE_NAME);
    path
}

pub fn load_history(readline: &mut Editor<(), DefaultHistory>) {
    let history_filepath = get_history_filepath();
    if !history_filepath.exists() {
        fs::File::create(&history_filepath).unwrap();
    }
    readline.load_history(&history_filepath).unwrap();
}
