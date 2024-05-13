use crate::chat::{
    gpt::GptReqParam,
    message::{Message, MessageRole},
    model::Model,
    role::Role,
    UserId,
};

use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// contains information like
/// - bot role
/// - user id
/// - target gpt model && setting
///
/// - conversation history
/// - current status
///
/// - last message timestamp
/// - token summary usage
#[derive(Deserialize, Serialize)]
pub struct Session {
    role: Role,
    user_id: UserId,
    model: Model,

    messages: Vec<Message>,
    status: Status,

    timestamp: DateTime,
    // token_usage,
}

#[derive(Deserialize, Serialize)]
pub enum Status {
    Replying,
    Replied,
}

impl Session {
    pub fn new(user_id: UserId) -> Self {
        let default_role = Role::Assistant;
        Self {
            role: default_role.clone(),
            user_id,
            model: Model::default(),
            messages: default_role.initial_message(),
            status: Status::Replied,
            timestamp: DateTime::now(),
        }
    }

    pub fn new_user_message(&mut self, content: String) -> anyhow::Result<()> {
        if !matches!(self.status, Status::Replied) {
            anyhow::bail!("status invalid.")
        }
        self.messages.push(Message::new(MessageRole::User, content));
        self.status = Status::Replying;
        Ok(())
    }

    pub fn generate_param(&mut self) -> GptReqParam {
        let body = json!({
            "messages" : self.messages,
            "model": self.model.model_str()
        })
        .to_string();
        GptReqParam {
            body,
            url: self.model.api_url(),
        }
    }

    pub fn recv_message(&mut self, message: &Message) {
        self.messages
            .push(Message::new(MessageRole::Assistant, message.content.clone()));
        self.status = Status::Replied;
    }
}
