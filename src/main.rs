mod config;
mod evaluator;
mod openai;
mod repl;

use crate::evaluator::evaluator;
use crate::repl::start_repl;

#[tokio::main]
async fn main() {
    println!("Hello!");
    config::Config::new();
    let client = openai::client();
    let evaluator = evaluator(&client);

    start_repl(evaluator).await;
}
