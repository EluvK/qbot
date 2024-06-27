use std::{sync::Arc, time::Duration};

use cqhttp_bot_frame::{
    bot::{Bot, Handler},
    RecvMsg, SendMsg,
};
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::info;

use crate::{
    chat::{ChatManager, ModelConfiguration},
    config::{AuthToken, QBotConfig},
    qbot_cmd::{ChatCommand, CommandChatMode},
};

pub struct QBot {
    _bot_send_tx: Arc<Sender<SendMsg>>, // might be useless
}

impl QBot {
    pub async fn new(config: QBotConfig) -> Self {
        let (instant_tx, instant_rx) = tokio::sync::mpsc::channel::<SendMsg>(10);
        let bot_send_tx = Arc::new(instant_tx); // maybe useless...

        let chat_manager = Arc::new(Mutex::new(ChatManager::new()));
        let message_handler = Arc::new(QBotMessageHandler::new(
            bot_send_tx.clone(),
            chat_manager.clone(),
            config.auth_token,
        ));
        let bot = Bot::new(config.cq_bot, message_handler, instant_rx).await;
        tokio::spawn(async move {
            bot.start().await;
        });

        Self {
            _bot_send_tx: bot_send_tx,
        }
    }
    pub async fn start(&self) -> ! {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }
}

struct QBotMessageHandler {
    _bot_instant_tx: Arc<Sender<SendMsg>>,
    chat_manager: Arc<Mutex<ChatManager>>,
    auth_token: AuthToken,
}

impl QBotMessageHandler {
    pub fn new(
        bot_instant_tx: Arc<Sender<SendMsg>>,
        chat_manager: Arc<Mutex<ChatManager>>,
        auth_token: AuthToken,
    ) -> Self {
        Self {
            _bot_instant_tx: bot_instant_tx,
            chat_manager,
            auth_token,
        }
    }

    pub async fn chat_msg(&self, msg: RecvMsg) -> anyhow::Result<SendMsg> {
        let (token, model_config) = match &self.auth_token {
            AuthToken::OpenAi(token) => (token, ModelConfiguration::openai()),
            AuthToken::DeepSeek(token) => (token, ModelConfiguration::deepseek_chat()),
        };

        let content = self
            .chat_manager
            .lock()
            .await
            .chat(msg.from_id, msg.content.clone(), token, model_config)
            .await?;
        Ok(msg.reply(content))
    }

    pub async fn chat_cmd(&self, cmd: ChatCommand, msg: RecvMsg) -> SendMsg {
        match cmd {
            ChatCommand::Clear => {
                self.chat_manager.lock().await.clear(msg.from_id);
                msg.reply("chat history cleared".into())
            }
            ChatCommand::Mode { mode } => {
                let mode_config = match mode {
                    CommandChatMode::Chat => ModelConfiguration::deepseek_chat(),
                    CommandChatMode::Code => ModelConfiguration::deepseek_code(),
                };
                self.chat_manager.lock().await.set_mode(msg.from_id, mode_config);
                msg.reply("chat mode changed".into())
            }
            ChatCommand::Info => {
                // let info = self.chat_manager.lock().await.info(msg.from_id);
                msg.reply("info todo".into())
            }
        }
    }
}

#[async_trait::async_trait]
impl Handler for QBotMessageHandler {
    type Cmd = crate::qbot_cmd::QBotCmd;
    async fn handle_msg(&self, msg: RecvMsg) -> Option<SendMsg> {
        match self.chat_msg(msg).await {
            Ok(msg) => Some(msg),
            Err(e) => {
                // debug
                info!("error: {e}");
                None
            }
        }
    }
    async fn handle_cmd(&self, cmd: Self::Cmd, msg: RecvMsg) -> Option<SendMsg> {
        if let Some(cmd) = cmd.sub {
            let msg = match cmd {
                crate::qbot_cmd::Commands::Chat(cmd) => self.chat_cmd(cmd, msg).await,
                crate::qbot_cmd::Commands::Nps { ip } => todo!(),
            };
            Some(msg)
        } else {
            None
        }
    }
    async fn check_cmd_auth(&self, cmd: &Self::Cmd, ori_msg: &RecvMsg, root_id: u64) -> bool {
        true
    }
}
