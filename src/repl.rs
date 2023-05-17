use crate::config::get_home_path;
use crate::evaluator::Evaluator;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::{fs, path::PathBuf, process};

pub async fn start_repl(mut evaluator: Evaluator<'_>) {
    let mut rl = Editor::<(), FileHistory>::new().unwrap();

    let mut history_filepath = init_history_filepath();
    rl.load_history(&mut history_filepath)
        .unwrap_or_else(|err| eprintln!("Failed to load history: {}", err));

    Evaluator::print_help();

    loop {
        let prompt = if evaluator.multi_line_mode {
            ">> "
        } else {
            "> "
        };
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line).unwrap();
                rl.save_history(&mut history_filepath).unwrap();
                evaluator.eval(line).await;
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C
                println!("Bye!");
                process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                println!("Bye!");
                process::exit(0);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
}

const HISTORY_FILE_NAME: &str = ".chatgpt-repl-history";

fn init_history_filepath() -> PathBuf {
    let mut filepath = get_home_path();
    filepath.push(HISTORY_FILE_NAME);
    if !filepath.exists() {
        fs::File::create(&filepath).unwrap();
    }

    filepath
}
