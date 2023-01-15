use crate::openai::completion;
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

async fn evaluator(openai_client: &OpenAIClient, line: &str) {
    if line.trim().len() == 0 {
        return;
    }
    let spinner = Spinner::new(Spinners::Dots, "Waiting for response...", Color::White);
    let completion_result = completion(openai_client, line).await;
    spinner.stop();

    match completion_result {
        Ok(response) => {
            println!("{}", response.choices[0].text);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}
