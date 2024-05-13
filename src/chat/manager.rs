use std::collections::HashMap;

use tracing::info;

use crate::{
    chat::{
        gpt::{GptRecvResult, GptReqParam},
        session::Session,
        UserId,
    },
    config::ApiKey,
};

pub struct ChatManager {
    api_key: ApiKey,
    sessions: HashMap<UserId, Session>,
}

impl ChatManager {
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            api_key,
            sessions: HashMap::new(),
        }
    }

    pub async fn chat(&mut self, user_id: UserId, msg: String) -> String {
        let session = self.sessions.entry(user_id).or_insert(Session::new(user_id));
        if let Err(e) = session.new_user_message(msg) {
            return format!("{e}, conflict");
        }
        let GptReqParam { body, url } = session.generate_param();

        let token = if url.contains("openai") {
            &self.api_key.open_ai
        } else if url.contains("deepseek") {
            &self.api_key.deep_seek
        } else {
            return format!("none api token for url:{url}");
        };

        match Self::send(body, url, token).await {
            Ok(resp) => {
                info!("recv resp: {resp:?}");

                if let Some(message) = resp.to_message() {
                    session.recv_message(&message);
                    message.content
                } else {
                    "gpt recv none result".to_string()
                }
            }
            Err(err) => {
                format!("chatbot send error: {err}")
            }
        }
    }

    async fn send(body: String, url: String, token: &str) -> anyhow::Result<GptRecvResult> {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .body(body)
            .send()
            .await
            .map_err(|err| anyhow::anyhow!("client error: {err}"))?
            .text()
            .await
            .map_err(|err| anyhow::anyhow!("client recv test err: {err}"))?;
        serde_json::from_str(&resp).map_err(|err| anyhow::anyhow!("client recv resp can not de {err}"))
    }
}
