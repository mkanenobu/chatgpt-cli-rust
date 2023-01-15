mod config;
mod evaluator;
mod openai;
mod repl;

use crate::evaluator::Evaluator;
use crate::repl::start_repl;

#[tokio::main]
async fn main() {
    config::Config::new().unwrap().init();

    let client = openai::client();
    let evaluator = Evaluator::new(&client);

    start_repl(evaluator).await;
}
