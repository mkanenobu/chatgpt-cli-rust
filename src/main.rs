mod config;
mod evaluator;
mod message;
mod openai;
mod repl;
mod set_api_key_prompt;

use crate::evaluator::Evaluator;
use crate::message::Messages;
use crate::repl::start_repl;
use crate::set_api_key_prompt::set_api_key_prompt;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let conf = config::Config::new();

    let mut msgs = Messages::new(&args.system_context);

    if args.set_api_key {
        set_api_key_prompt();
    } else if let Some(api_key) = conf.api_key {
        let client = openai::client(api_key);
        let evaluator = Evaluator::new(&client, &mut msgs);

        start_repl(evaluator).await;
    } else {
        println!("API Key is not set.");
        set_api_key_prompt();
    }
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Set API Key
    #[arg(long)]
    set_api_key: bool,

    /// System context
    #[arg(long)]
    system_context: Option<String>,
}
