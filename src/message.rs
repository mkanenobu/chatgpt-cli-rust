use async_openai::types as openai;

pub fn create_system_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::System(openai::ChatCompletionRequestSystemMessage {
        content: content.to_string(),
        role: Default::default(),
        name: None,
    })
}

#[allow(deprecated)]
pub fn create_assistant_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::Assistant(openai::ChatCompletionRequestAssistantMessage {
        content: Some(content.to_string()),
        role: Default::default(),
        tool_calls: None,
        name: None,

        // Deprecated
        function_call: None,
    })
}

pub fn create_text_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::User(openai::ChatCompletionRequestUserMessage {
        content: openai::ChatCompletionRequestUserMessageContent::Text(
            content.to_string(),
        ),
        role: Default::default(),
        name: None,
    })
}
