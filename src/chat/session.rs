use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::chat::{
    gpt::{GptRecvResult, GptReqParam},
    message::{Message, MessageRole},
    model::ModelConfiguration,
    role::Role,
    UserId,
};

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
    model_config: ModelConfiguration,

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
    pub fn new(user_id: UserId, model_config: ModelConfiguration) -> Self {
        let default_role = Role::Assistant;
        Self {
            role: default_role.clone(),
            user_id,
            model_config,
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
            "model": self.model_config.model
        })
        .to_string();
        GptReqParam {
            body,
            url: self.model_config.api_url(),
        }
    }

    pub fn recv_message(&mut self, gpt_result: GptRecvResult) -> String {
        info!("[COST] finish api call {}", gpt_result.usage);
        let content = gpt_result
            .choices
            .into_iter()
            .nth(0)
            .map_or("gpt return no choices".to_owned(), |choice| choice.message.content);
        self.messages
            .push(Message::new(MessageRole::Assistant, content.clone()));
        self.status = Status::Replied;
        content
    }

    pub fn clear_message(&mut self) {
        self.messages.clear();
        self.status = Status::Replied;
    }

    pub fn set_mode(&mut self, model_config: ModelConfiguration) {
        self.model_config = model_config;
    }
}
