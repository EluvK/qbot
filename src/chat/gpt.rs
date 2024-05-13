pub(crate) use req::GptReqParam;
pub(crate) use resp::GptRecvResult;

pub(crate) mod req {
    pub(crate) struct GptReqParam {
        pub(crate) body: String,
        pub(crate) url: String,
    }
}

pub(crate) mod resp {
    use serde::Deserialize;
    use std::fmt::Display;
    use tracing::info;

    #[derive(Debug, Deserialize)]
    struct Usage {
        prompt_tokens: i64,
        completion_tokens: i64,
        total_tokens: i64,
    }

    impl Display for Usage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Prompt tokens used: {}, Completion tokens used: {}, Total tokens used: {}",
                self.prompt_tokens, self.completion_tokens, self.total_tokens
            )
        }
    }

    #[derive(Debug, Deserialize)]
    struct Message {
        role: crate::chat::message::MessageRole,
        content: String,
    }

    #[derive(Debug, Deserialize)]
    struct Choices {
        message: Message,
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    pub struct GptRecvResult {
        id: String,
        object: String,
        created: u64,
        model: String,
        choices: Vec<Choices>,
        usage: Usage,
    }

    impl GptRecvResult {
        pub fn to_message(&self) -> Option<crate::chat::message::Message> {
            info!("[COST] finish chatgpt-api call {}", self.usage);
            if self.choices.is_empty() {
                None
            } else {
                Some(crate::chat::message::Message::new(
                    self.choices[0].message.role.clone(),
                    self.choices[0].message.content.clone(),
                ))
            }
        }
    }
}
