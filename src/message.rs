use async_openai::types::{ChatCompletionRequestMessage, Role};
use std::fmt;

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

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
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
