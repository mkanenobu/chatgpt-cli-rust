use anyhow::{Context, Result};
use async_openai::types::{
    ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
};
use async_openai::Client;

pub fn client(api_key: String) -> Client {
    Client::new().with_api_key(api_key)
}

#[allow(dead_code)]
pub async fn completion(
    client: &Client,
    args: CreateChatCompletionRequest,
) -> Result<CreateChatCompletionResponse> {
    client
        .chat()
        .create(args)
        .await
        .context("Failed to create completion")
}

pub async fn completion_stream(
    client: &Client,
    args: CreateChatCompletionRequest,
) -> Result<ChatCompletionResponseStream> {
    client
        .chat()
        .create_stream(args)
        .await
        .context("Failed to create completion stream")
}
