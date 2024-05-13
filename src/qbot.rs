use std::{sync::Arc, time::Duration};

use cqhttp_bot_frame::{
    bot::{Bot, Handler},
    RecvMsg, SendMsg,
};
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{chat::ChatManager, config::QBotConfig};

pub struct QBot {
    _bot_send_tx: Arc<Sender<SendMsg>>, // might be useless
}

impl QBot {
    pub async fn new(config: QBotConfig) -> Self {
        let (instant_tx, instant_rx) = tokio::sync::mpsc::channel::<SendMsg>(10);
        let bot_send_tx = Arc::new(instant_tx); // maybe useless...

        let chat_manager = Arc::new(Mutex::new(ChatManager::new(config.api_key)));
        let message_handler = Arc::new(QBotMessageHandler::new(bot_send_tx.clone(), chat_manager.clone()));
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

pub struct QBotMessageHandler {
    pub(crate) _bot_instant_tx: Arc<Sender<SendMsg>>,
    pub(crate) chat_manager: Arc<Mutex<ChatManager>>,
}

impl QBotMessageHandler {
    pub fn new(bot_instant_tx: Arc<Sender<SendMsg>>, chat_manager: Arc<Mutex<ChatManager>>) -> Self {
        Self {
            _bot_instant_tx: bot_instant_tx,
            chat_manager,
        }
    }

    pub async fn chat(&self, msg: RecvMsg) -> Option<SendMsg> {
        let content = self
            .chat_manager
            .lock()
            .await
            .chat(msg.from_id, msg.content.clone())
            .await;
        Some(msg.reply(content))
    }
}

#[async_trait::async_trait]
impl Handler for QBotMessageHandler {
    type Cmd = crate::qbot_cmd::QBotCmd;
    async fn handle_msg(&self, msg: RecvMsg) -> Option<SendMsg> {
        self.chat(msg).await
    }
    async fn handle_cmd(&self, cmd: Self::Cmd, msg: RecvMsg) -> Option<SendMsg> {
        None
    }
    async fn check_cmd_auth(&self, cmd: &Self::Cmd, ori_msg: &RecvMsg, root_id: u64) -> bool {
        true
    }
}
