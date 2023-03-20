use crate::message::{create_message, Messages};
use crate::openai::completion_stream;
use anyhow::{anyhow, Result};
use async_openai::types::{
    CreateChatCompletionRequest, CreateChatCompletionRequestArgs, Role as MessageRole,
};
use async_openai::Client as OpenAIClient;
use futures::StreamExt;
use spinoff::{spinners, Spinner};
use std::io::{stdout, Write};

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
    messages: &'a mut Messages,
    pub multi_line_mode: bool,
    multi_line_mode_message_stack: Vec<String>,
    #[allow(dead_code)]
    config: EvaluatorConfig,
}

pub struct EvaluatorConfig {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
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
                self.openai_completion_stream().await.unwrap();
            }
            _ => {
                if self.multi_line_mode {
                    self.multi_line_mode_message_stack.push(line.to_string());
                    return;
                }

                self.messages.push(create_message(line, MessageRole::User));
                self.openai_completion_stream().await.unwrap();
            }
        }
    }

    async fn openai_completion_stream(&mut self) -> Result<()> {
        // FIXME: たぶん stdout のロックのせいでスピナーが動かない
        let spinner = Spinner::new(spinners::Dots, "Waiting for OpenAI response...", None);
        let completion_args = self.build_completion_args()?;
        let completion_stream = completion_stream(self.openai_client, completion_args).await;
        spinner.clear();

        match completion_stream {
            Ok(mut stream) => {
                let mut stdout = stdout().lock();

                while let Some(msg) = stream.next().await {
                    match msg {
                        Ok(response) => {
                            response.choices.iter().for_each(|chat_choice| {
                                if let Some(ref content) = chat_choice.delta.content {
                                    write!(stdout, "{}", content).unwrap();
                                    stdout.flush().unwrap();
                                }
                            });
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    }
                }
                writeln!(stdout).unwrap();
                stdout.flush().unwrap();
            }
            Err(err) => {
                println!("Error: {:?}", err);
                self.messages.pop();
            }
        }
        Ok(())
    }

    fn build_completion_args(&self) -> Result<CreateChatCompletionRequest> {
        CreateChatCompletionRequestArgs::default()
            .model(
                self.config
                    .model
                    .clone()
                    .unwrap_or("gpt-3.5-turbo".to_string()),
            )
            .temperature(self.config.temperature.unwrap_or(0.7))
            .top_p(self.config.top_p.unwrap_or(1.0))
            .messages(self.messages.messages.as_ref())
            .build()
            .map_err(|err| anyhow!(err))
    }
}
