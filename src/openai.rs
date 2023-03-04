use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    Role,
};
use async_openai::Client;
use std::fmt::{Display, Formatter};

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

pub fn create_message(content: &str, role: Role) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage {
        content: content.to_string(),
        role,
        name: None,
    }
}

#[derive(Debug)]
pub struct Messages {
    pub messages: Vec<ChatCompletionRequestMessage>,
}

impl Display for Messages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let contents = self
            .messages
            .iter()
            .map(|msg| msg.content.clone())
            .collect::<Vec<String>>();
        write!(f, "{}", serde_json::to_string_pretty(&contents).unwrap())
    }
}

impl Messages {
    pub fn new(system_context_message: &Option<String>) -> Messages {
        let mut msgs = Messages { messages: vec![] };
        if let Some(msg) = system_context_message {
            msgs.push(create_message(msg, Role::System));
        }
        msgs
    }

    pub fn push(&mut self, msg: ChatCompletionRequestMessage) {
        self.messages.push(msg);
    }

    pub fn pop(&mut self) -> Option<ChatCompletionRequestMessage> {
        self.messages.pop()
    }
}
