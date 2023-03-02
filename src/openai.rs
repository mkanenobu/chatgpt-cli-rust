use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    Role,
};
use async_openai::Client;

pub fn client(api_key: String) -> Client {
    Client::new().with_api_key(api_key)
}

pub async fn completion(
    client: &Client,
    prompt: &str,
) -> Result<CreateChatCompletionResponse, OpenAIError> {
    let message = ChatCompletionRequestMessage {
        content: prompt.to_string(),
        role: Role::User,
        name: None,
    };
    // message.content = prompt.to_string();

    let args = CreateChatCompletionRequestArgs::default()
        .messages(vec![message])
        .model("gpt-3.5-turbo")
        .temperature(0.7)
        .build()?;
    client.chat().create(args).await
}
