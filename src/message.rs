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
    pub system_context: Option<ChatCompletionRequestMessage>,
    pub messages: Vec<ChatCompletionRequestMessage>,
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msgs = self.messages.clone();
        if let Some(system_context) = &self.system_context {
            msgs.insert(0, system_context.clone());
        }
        write!(f, "{}", serde_json::to_string_pretty(&msgs).unwrap())
    }
}

impl Messages {
    pub fn new(system_context_message: Option<String>) -> Messages {
        Messages {
            messages: vec![],
            system_context: system_context_message.map(|msg| create_message(&msg, Role::System)),
        }
    }

    pub fn push(&mut self, msg: ChatCompletionRequestMessage) {
        self.messages.push(msg);
    }

    pub fn pop(&mut self) -> Option<ChatCompletionRequestMessage> {
        self.messages.pop()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn merged_messages(&self) -> Vec<ChatCompletionRequestMessage> {
        let mut msgs = self.messages.clone();
        if let Some(system_context) = &self.system_context {
            msgs.insert(0, system_context.clone());
        }
        msgs
    }
}
