mod config;
mod evaluator;
mod openai;
mod repl;

use crate::evaluator::Evaluator;
use crate::repl::start_repl;

#[tokio::main]
async fn main() {
    let conf = config::Config::new().unwrap();

    let client = openai::client(conf.api_key);
    let evaluator = Evaluator::new(&client);

    start_repl(evaluator).await;
}
