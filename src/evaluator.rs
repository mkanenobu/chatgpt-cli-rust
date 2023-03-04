use crate::openai::{completion, create_message, Messages};
use async_openai::types::{CreateChatCompletionResponse, Role as MessageRole};
use async_openai::Client as OpenAIClient;
use async_trait::async_trait;
use spinoff::{Color, Spinner, Spinners};

#[async_trait]
pub trait Eval {
    async fn eval(&mut self, line: &str);
}

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
    messages: &'a mut Messages,
}

impl<'a> Evaluator<'a> {
    pub fn new(openai_client: &'a OpenAIClient, messages: &'a mut Messages) -> Evaluator<'a> {
        Evaluator {
            openai_client,
            messages,
        }
    }

    fn show_messages_stack(&self) {
        println!("{}", &self.messages);
    }
}

#[async_trait]
impl<'a> Eval for Evaluator<'a> {
    async fn eval(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        match line {
            "?messages" => {
                self.show_messages_stack();
            }
            _ => {
                openai_completion(self.openai_client, line, self.messages).await;
            }
        }
    }
}

async fn openai_completion(openai_client: &OpenAIClient, line: &str, messages: &mut Messages) {
    messages.push(create_message(line, MessageRole::User));

    let spinner = Spinner::new(Spinners::Dots, "Waiting for response...", Color::White);
    let completion_result = completion(openai_client, messages).await;
    spinner.stop();

    match completion_result {
        Ok(response) => {
            println!("{}", format_response(response));
        }
        Err(err) => {
            println!("Error: {}", err);
            messages.pop();
        }
    };
}

fn format_response(response: CreateChatCompletionResponse) -> String {
    response
        .choices
        .iter()
        .map(|choice| choice.message.content.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}
