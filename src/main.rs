mod config;
mod evaluator;
mod openai;
mod repl;
mod set_api_key_prompt;

use crate::evaluator::Evaluator;
use crate::repl::start_repl;
use crate::set_api_key_prompt::set_api_key_prompt;

#[tokio::main]
async fn main() {
    let conf = config::Config::new();

    if let Some(api_key) = conf.api_key {
        let client = openai::client(api_key);
        let evaluator = Evaluator::new(&client);

        start_repl(evaluator).await;
    } else {
        println!("API Key is not set.");
        set_api_key_prompt();
    }
}
