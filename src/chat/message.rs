use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role,
            content,
            timestamp: DateTime::now(),
        }
    }
}
