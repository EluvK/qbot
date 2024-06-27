use serde::{Deserialize, Serialize};

use crate::chat::message::{Message, MessageRole};

#[derive(Clone, Deserialize, Serialize)]
pub enum Role {
    Assistant,
}

impl Role {
    pub fn initial_message(&self) -> Vec<Message> {
        match self {
            Role::Assistant => vec![Message::new(MessageRole::System, ASSISTANT_SYSTEM_INFO.into())],
        }
    }
}

static ASSISTANT_SYSTEM_INFO :&str = "You are a helpful assistant. Unless there is a specific word count requirement, please answer the question as concisely as possible";
