use crate::evaluator::Evaluator;
use crate::history::load_history;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::{env, fs, path::PathBuf, process};

pub async fn start_repl(mut evaluator: Evaluator<'_>) {
    let mut rl = Editor::<(), FileHistory>::new().unwrap();

    load_history(&mut rl);
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
                rl.save_history(&history_filepath).unwrap();
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
