use async_openai::types as openai;

pub fn create_system_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::System(openai::ChatCompletionRequestSystemMessage {
        content: Some(content.to_string()),
        role: Default::default(),
    })
}

#[allow(deprecated)]
pub fn create_assistant_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::Assistant(openai::ChatCompletionRequestAssistantMessage {
        content: Some(content.to_string()),
        role: Default::default(),
        tool_calls: None,

        // Deprecated
        function_call: None,
    })
}

pub fn create_text_message(content: &str) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::User(openai::ChatCompletionRequestUserMessage {
        content: Some(openai::ChatCompletionRequestUserMessageContent::Text(
            content.to_string(),
        )),
        role: Default::default(),
    })
}

#[allow(dead_code)]
pub fn create_text_message_with_image(
    text_content: &str,
    image_content: &str,
) -> openai::ChatCompletionRequestMessage {
    openai::ChatCompletionRequestMessage::User(openai::ChatCompletionRequestUserMessage {
        content: Some(openai::ChatCompletionRequestUserMessageContent::Array(
            vec![
                openai::ChatCompletionRequestMessageContentPart::Text(
                    openai::ChatCompletionRequestMessageContentPartText {
                        r#type: Default::default(),
                        text: text_content.to_string(),
                    },
                ),
                openai::ChatCompletionRequestMessageContentPart::Image(
                    openai::ChatCompletionRequestMessageContentPartImage {
                        r#type: Default::default(),
                        image_url: openai::ImageUrl::from(image_content.to_string()),
                    },
                ),
            ],
        )),
        role: Default::default(),
    })
}
