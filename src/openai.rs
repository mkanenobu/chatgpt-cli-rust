use async_openai::error::OpenAIError;
use async_openai::types::{CreateCompletionRequestArgs, CreateCompletionResponse};
use async_openai::Client;

pub fn client() -> Client {
    Client::new()
}

pub async fn completion(
    client: &Client,
    prompt: &str,
) -> Result<CreateCompletionResponse, OpenAIError> {
    let args = CreateCompletionRequestArgs::default()
        .prompt(prompt)
        .model("text-davinci-003")
        .temperature(0.7)
        .max_tokens(512u16)
        .build()?;
    client.completions().create(args).await
}
