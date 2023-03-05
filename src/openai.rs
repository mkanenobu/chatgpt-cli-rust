use crate::message::Messages;
use async_openai::error::OpenAIError;
use async_openai::types::{CreateChatCompletionRequestArgs, CreateChatCompletionResponse};
use async_openai::Client;

pub fn client(api_key: String) -> Client {
    Client::new().with_api_key(api_key)
}

pub async fn completion(
    client: &Client,
    messages: &Messages,
) -> Result<CreateChatCompletionResponse, OpenAIError> {
    let args = CreateChatCompletionRequestArgs::default()
        .messages(messages.messages.clone())
        .model("gpt-3.5-turbo")
        .temperature(0.7)
        .build()?;
    client.chat().create(args).await
}
