mod config;
mod evaluator;
mod openai;
mod repl;

use crate::evaluator::Eval;
use crate::repl::start_repl;

#[tokio::main]
async fn main() {
    config::Config::new().init();

    let client = openai::client();
    let evaluator = Eval::new(&client);

    start_repl(evaluator).await;
}
