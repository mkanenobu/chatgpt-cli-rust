use anyhow::{Context, Result};
use async_openai::types::{
    ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
};
use async_openai::{config::OpenAIConfig, Client};

pub type OpenAIClient = Client<OpenAIConfig>;

pub fn client(api_key: String) -> OpenAIClient {
    let config = OpenAIConfig::new().with_api_key(api_key);
    Client::with_config(config)
}

#[allow(dead_code)]
pub async fn completion(
    client: &OpenAIClient,
    args: CreateChatCompletionRequest,
) -> Result<CreateChatCompletionResponse> {
    client
        .chat()
        .create(args)
        .await
        .context("Failed to create completion")
}

pub async fn completion_stream(
    client: &OpenAIClient,
    args: CreateChatCompletionRequest,
) -> Result<ChatCompletionResponseStream> {
    client
        .chat()
        .create_stream(args)
        .await
        .context("Failed to create completion stream")
}
