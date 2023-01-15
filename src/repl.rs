use crate::evaluator::Eval;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process;

pub async fn start_repl(evaluator: impl Eval) {
    let mut rl = Editor::<()>::new().unwrap();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                evaluator.eval(line.as_str()).await;
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
