use crate::evaluator::Evaluator;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{env, fs, path::PathBuf, process};

fn history_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".chatgpt-repl-history");
    if !path.exists() {
        fs::File::create(&path).unwrap();
    };
    path
}

pub async fn start_repl<'a>(mut evaluator: Evaluator<'a>) {
    let mut rl = Editor::<()>::new().unwrap();
    let filepath = history_filepath();
    rl.load_history(&filepath).unwrap();
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
                rl.add_history_entry(line);
                evaluator.eval(line).await;
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C
                rl.save_history(&filepath).unwrap();
                println!("Bye!");
                process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                rl.save_history(&filepath).unwrap();
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
