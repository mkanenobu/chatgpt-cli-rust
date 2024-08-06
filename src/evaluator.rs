use crate::message::{create_assistant_message, create_text_message};
use crate::messages::Messages;
use crate::openai::{completion_stream, OpenAIClient};
use crate::tokenizer::get_text_token_count;
use anyhow::{anyhow, Result};
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionRequestArgs};
use futures::prelude::*;
use spinoff::{spinners, Color, Spinner};
use std::io::{stdout, Write};
use stream_cancel::{StreamExt, Tripwire};

pub struct Evaluator<'a> {
    openai_client: &'a OpenAIClient,
    messages: &'a mut Messages,
    pub multi_line_mode: bool,
    multi_line_mode_message_stack: Vec<String>,
    #[allow(dead_code)]
    config: EvaluatorConfig,
}

#[derive(Debug)]
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
.clear                  # Clear messages stack
.messages               # Show messages stack
.enable-multiline-mode  # Enable multi-line mode
.disable-multiline-mode # Disable multi-line mode
.config                 # Show config
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
            ".config" => {
                println!("Config: {:#?}", &self.config);
            }
            ".send" => {
                if !self.multi_line_mode {
                    println!("Multi-line mode is not enabled.");
                    return;
                }
                let message = self.multi_line_mode_message_stack.join("\n");

                let token_count = get_text_token_count(&self.model(), line);
                if let Ok(count) = token_count {
                    println!("Token count: {}", count);
                }

                self.messages.push(create_text_message(&message));
                self.multi_line_mode_message_stack = vec![];
                let response = self.openai_completion_stream().await.unwrap();

                let token_count = get_text_token_count(&self.model(), &response);
                if let Ok(count) = token_count {
                    println!("Response token count: {}", count);
                }

                self.messages.push(create_assistant_message(&response));
            }
            ".clear" => {
                self.messages.clear();
                println!("Clear message stack");
            }
            _ => {
                if self.multi_line_mode {
                    self.multi_line_mode_message_stack.push(line.to_string());
                    return;
                }

                let token_count = get_text_token_count(&self.model(), line);
                if let Ok(count) = token_count {
                    println!("Token count: {}", count);
                }

                self.messages.push(create_text_message(line));
                let response = self.openai_completion_stream().await.unwrap();

                let token_count = get_text_token_count(&self.model(), &response);
                if let Ok(count) = token_count {
                    println!("Response token count: {}", count);
                }

                self.messages.push(create_assistant_message(&response));
            }
        }
    }

    fn model(&self) -> String {
        self.config
            .model
            .clone()
            .unwrap_or("gpt-3.5-turbo".to_string())
    }

    async fn openai_completion_stream(&mut self) -> Result<String> {
        // FIXME: completion_stream 実行中にスピナーが止まる
        let mut spinner = Spinner::new(
            spinners::Dots,
            "Waiting for OpenAI response...",
            Color::Blue,
        );

        let completion_args = self.build_completion_args()?;
        let completion_stream = completion_stream(self.openai_client, completion_args).await;
        spinner.clear();

        let mut buf = String::new();
        match completion_stream {
            Ok(stream) => {
                let mut stdout = stdout().lock();
                let (trigger, tripwire) = Tripwire::new();

                let mut stream = stream.take_until_if(tripwire);
                tokio::spawn(async move {
                    tokio::signal::ctrl_c().await.unwrap();
                    drop(trigger);
                });

                while let Some(msg) = stream.next().await {
                    match msg {
                        Ok(response) => {
                            response.choices.iter().for_each(|chat_choice| {
                                if let Some(ref content) = chat_choice.delta.content {
                                    buf.push_str(content);
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
        Ok(buf)
    }

    fn build_completion_args(&self) -> Result<CreateChatCompletionRequest> {
        CreateChatCompletionRequestArgs::default()
            .model(self.model())
            .temperature(self.config.temperature.unwrap_or(0.7))
            .top_p(self.config.top_p.unwrap_or(1.0))
            .messages(self.messages.merged_messages())
            .build()
            .map_err(|err| anyhow!(err))
    }
}
