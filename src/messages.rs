use crate::message::create_system_message;
use async_openai::types::ChatCompletionRequestMessage;
use std::fmt;

#[derive(Debug)]
pub struct Messages {
    pub system_context: Option<ChatCompletionRequestMessage>,
    pub messages: Vec<ChatCompletionRequestMessage>,
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut messages = self.messages.clone();
        if let Some(system_context) = &self.system_context {
            messages.insert(0, system_context.clone());
        }
        write!(f, "{}", serde_json::to_string_pretty(&messages).unwrap())
    }
}

impl Messages {
    pub fn new(system_context_message: Option<String>) -> Messages {
        Messages {
            messages: vec![],
            system_context: system_context_message.map(|msg| create_system_message(&msg)),
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
        let mut messages = self.messages.clone();
        if let Some(system_context) = &self.system_context {
            messages.insert(0, system_context.clone());
        }
        messages
    }
}
