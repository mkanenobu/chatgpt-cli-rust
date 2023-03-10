use crate::message::{create_message, Messages};
use crate::openai::completion;
use crate::say::say;
use async_openai::types::{CreateChatCompletionResponse, Role as MessageRole};
use async_openai::Client as OpenAIClient;
use spinoff::{Color, Spinner, Spinners};
use tokio::process::Child;

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
    messages: &'a mut Messages,
    pub multi_line_mode: bool,
    multi_line_mode_message_stack: Vec<String>,
    config: EvaluatorConfig,
    say_child_process: Option<Child>,
}

pub struct EvaluatorConfig {
    pub say: bool,
}

impl<'a> Evaluator<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        messages: &'a mut Messages,
        config: EvaluatorConfig,
    ) -> Evaluator<'a> {
        Evaluator {
            openai_client,
            messages,
            multi_line_mode: false,
            multi_line_mode_message_stack: vec![],
            config,
            say_child_process: None,
        }
    }

    fn show_messages_stack(&self) {
        println!("{}", &self.messages);
    }

    pub fn print_help() {
        println!("{}", HELP.trim());
    }
}

const HELP: &str = "
.help                   # Show this help
.messages               # Show messages stack
.enable-multiline-mode  # Enable multi-line mode
.disable-multiline-mode # Disable multi-line mode
.send                   # Send multi-line message
";

impl<'a> Evaluator<'a> {
    pub async fn eval(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        match line {
            ".help" => {
                Evaluator::print_help();
            }
            ".messages" => {
                self.show_messages_stack();
            }
            ".enable-multiline-mode" => {
                self.multi_line_mode = true;
                println!("Multi-line mode enabled.");
            }
            ".disable-multiline-mode" => {
                self.multi_line_mode = false;
                println!("Multi-line mode disabled.");
            }
            ".send" => {
                if !self.multi_line_mode {
                    println!("Multi-line mode is not enabled.");
                    return;
                }
                let message = self.multi_line_mode_message_stack.join("\n");
                self.messages
                    .push(create_message(&message, MessageRole::User));
                self.multi_line_mode_message_stack = vec![];
                self.openai_completion().await;
            }
            _ => {
                if self.multi_line_mode {
                    self.multi_line_mode_message_stack.push(line.to_string());
                    return;
                }

                self.messages.push(create_message(line, MessageRole::User));
                self.openai_completion().await;
            }
        }
    }

    async fn openai_completion(&mut self) {
        let spinner = Spinner::new(Spinners::Dots, "Waiting for response...", Color::White);
        let completion_result = completion(self.openai_client, self.messages).await;
        spinner.stop();

        match completion_result {
            Ok(response) => {
                let response = format_response(response);
                if let Some(mut child_process) = self.say_child_process.take() {
                    let _ = child_process.kill().await;
                }
                if self.config.say {
                    let child_process = say(&response).await;
                    self.say_child_process = Some(child_process);
                }
                println!("{}", response);
            }
            Err(err) => {
                println!("Error: {}", err);
                self.messages.pop();
            }
        };
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
