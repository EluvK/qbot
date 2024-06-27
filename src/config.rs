use std::path::Path;

use anyhow::Context;
use cqhttp_bot_frame::bot::BotConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum AuthToken {
    OpenAi(String),
    DeepSeek(String),
}

#[derive(Debug, Deserialize)]
pub struct QBotConfig {
    pub cq_bot: BotConfig,
    // pub proxy: Option<String>,
    pub auth_token: AuthToken,
}

pub fn load_from_file(path: &Path) -> anyhow::Result<QBotConfig> {
    config::Config::builder()
        .add_source(config::File::from(path))
        .build()
        .with_context(|| format!("failed to load configuration from {}", path.display()))?
        .try_deserialize()
        .context("failed to deserialize configuration")
}

pub fn default_config() -> String {
    r#"---
cq_bot:
  websocket: ws://localhost:8080/ws
  bot_qq: 123
  root_qq: 456
proxy: 
auth_token:
  DeepSeek: sksksksk
"#
    .into()
}
