mod config;
mod evaluator;
mod openai;
mod repl;

use crate::repl::start_repl;

#[tokio::main]
async fn main() {
    config::Config::new();
    let client = openai::client();

    start_repl(&client).await;
}
