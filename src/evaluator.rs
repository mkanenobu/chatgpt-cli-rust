use crate::message::{create_message, Messages};
use crate::openai::completion_stream;
use async_openai::types::Role as MessageRole;
use async_openai::Client as OpenAIClient;
use futures::StreamExt;
use std::io::{stdout, Write};

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
    messages: &'a mut Messages,
    pub multi_line_mode: bool,
    multi_line_mode_message_stack: Vec<String>,
    #[allow(dead_code)]
    config: EvaluatorConfig,
}

// TODO: engine, temperature などを設定ファイルから指定できるようにする
pub struct EvaluatorConfig {}

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
                self.openai_completion_stream().await;
            }
            _ => {
                if self.multi_line_mode {
                    self.multi_line_mode_message_stack.push(line.to_string());
                    return;
                }

                self.messages.push(create_message(line, MessageRole::User));
                self.openai_completion_stream().await;
            }
        }
    }

    async fn openai_completion_stream(&mut self) {
        let completion_stream = completion_stream(self.openai_client, self.messages).await;

        let mut lock = stdout().lock();
        match completion_stream {
            Ok(mut stream) => {
                while let Some(msg) = stream.next().await {
                    match msg {
                        Ok(response) => {
                            response.choices.iter().for_each(|chat_choice| {
                                if let Some(ref content) = chat_choice.delta.content {
                                    write!(lock, "{}", content).unwrap();
                                    lock.flush().unwrap();
                                }
                            });
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    }
                }
                write!(lock, "\n").unwrap();
                lock.flush().unwrap();
            }
            Err(err) => {
                println!("Error: {:?}", err);
                self.messages.pop();
            }
        }
    }
}
