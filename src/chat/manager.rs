use std::collections::HashMap;

use crate::chat::{
    gpt::{send_to_gpt, GptReqParam},
    session::Session,
    ModelConfiguration, UserId,
};

pub struct ChatManager {
    sessions: HashMap<UserId, Session>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub async fn chat(
        &mut self,
        user_id: UserId,
        msg: String,
        token: &str,
        model_config: ModelConfiguration,
    ) -> anyhow::Result<String> {
        let session = self
            .sessions
            .entry(user_id)
            .or_insert(Session::new(user_id, model_config));
        session.new_user_message(msg)?;
        let GptReqParam { body, url } = session.generate_param();

        let recv_message = send_to_gpt(body, url, token).await?;
        let content = session.recv_message(recv_message);
        Ok(content)
    }

    pub fn clear(&mut self, user_id: UserId) {
        self.sessions.entry(user_id).and_modify(|s| s.clear_message());
    }

    pub fn set_mode(&mut self, user_id: UserId, mode_config: ModelConfiguration) {
        self.sessions.entry(user_id).and_modify(|s| s.set_mode(mode_config));
    }
}
