use crate::openai::completion;
use async_openai::types::CreateChatCompletionResponse;
use async_openai::Client as OpenAIClient;
use async_trait::async_trait;
use spinoff::{Color, Spinner, Spinners};

#[async_trait]
pub trait Eval {
    async fn eval(&self, line: &str);
}

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
}

impl<'a> Evaluator<'a> {
    pub fn new(openai_client: &'a OpenAIClient) -> Evaluator<'a> {
        Evaluator { openai_client }
    }
}

#[async_trait]
impl<'a> Eval for Evaluator<'a> {
    async fn eval(&self, line: &str) {
        evaluator(self.openai_client, line).await
    }
}

fn format_response(response: CreateChatCompletionResponse) -> String {
    response
        .choices
        .iter()
        .map(|choice| choice.message.content.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

async fn evaluator(openai_client: &OpenAIClient, line: &str) {
    if line.trim().is_empty() {
        return;
    }
    let spinner = Spinner::new(Spinners::Dots, "Waiting for response...", Color::White);
    let completion_result = completion(openai_client, line).await;
    spinner.stop();

    match completion_result {
        Ok(response) => {
            println!("{}", format_response(response));
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}
