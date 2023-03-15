use crate::message::Messages;
use anyhow::{Context, Result};
use async_openai::types::{
    ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse,
};
use async_openai::Client;

pub fn client(api_key: String) -> Client {
    Client::new().with_api_key(api_key)
}

fn build_completion_args(messages: &Messages) -> Result<CreateChatCompletionRequest> {
    CreateChatCompletionRequestArgs::default()
        .messages(messages.messages.clone())
        .model("gpt-3.5-turbo")
        .temperature(0.7)
        .build()
        .context("Failed to build completion args")
}

#[allow(dead_code)]
pub async fn completion(
    client: &Client,
    messages: &Messages,
) -> Result<CreateChatCompletionResponse> {
    let args = build_completion_args(messages)?;
    client
        .chat()
        .create(args)
        .await
        .context("Failed to create completion")
}

pub async fn completion_stream(
    client: &Client,
    messages: &Messages,
) -> Result<ChatCompletionResponseStream> {
    let args = build_completion_args(messages)?;
    client
        .chat()
        .create_stream(args)
        .await
        .context("Failed to create completion stream")
}
