use std::rc::Rc;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    config::Config,
    cqbot::msg::{MessageType, RecvMsg, SendMsg},
    gpt::{GPTPostMessage, GPTRecvMessage},
    private_manager::ChatManager,
};

pub struct Bot {
    web_socket_stream: Box<WebSocketStream<MaybeTlsStream<TcpStream>>>,

    chat_manager: ChatManager,
    config: Rc<Config>,
}

static OPENAIAPIURL: &str = "https://api.openai.com/v1/chat/completions";

impl Bot {
    pub async fn new(config: Config) -> Self {
        let (wss, _) = connect_async(config.url()).await.expect("Fail to connect");
        let bot_config = Rc::new(config);
        Self {
            web_socket_stream: Box::new(wss),
            chat_manager: ChatManager::new(Rc::clone(&bot_config)),
            config: bot_config,
        }
    }

    pub async fn loop_read(&mut self) -> ! {
        loop {
            let msg = self
                .web_socket_stream
                .next()
                .await
                .expect("Fail to read message")
                .expect("Message error");
            // println!("Received message: {:?}", msg);
            if let Some(send_msg) = self.handle_cqhttp_message(msg).await {
                self.send_back_message(send_msg).await
            }
        }
    }

    async fn handle_cqhttp_message(&mut self, message: Message) -> Option<SendMsg> {
        if let Message::Text(text) = message {
            // println!("Handle Text:{}", &text);
            let msg = serde_json::from_str::<RecvMsg>(&text).ok()?;
            let (is_at, msg) = msg.pre_parse_msg(self.config.bot_qq());
            println!("is_at:{:?}, RecvMessage:{:?}", is_at, &msg);
            if &MessageType::Group == msg.message_type() && !is_at {
                None
            } else {
                self.handle_message(msg).await
            }
        } else {
            None
        }
    }
    async fn send_back_message(&mut self, send_msg: SendMsg) {
        // println!("reply message: {:?}", send_msg);
        let reply = Message::Text(serde_json::to_string(&send_msg).unwrap());
        self.web_socket_stream.send(reply).await.expect("Send_fail");
    }

    async fn handle_message(&mut self, message: RecvMsg) -> Option<SendMsg> {
        let group_id = if message.message_type() == &MessageType::Group {
            Some(message.group_id())
        } else {
            None
        };
        let user_id = message.user_id();
        let ts = message.message_ts();

        let pm_result = self
            .chat_manager
            .pre_handle_private_message(group_id, user_id, ts, message.message().clone());
        match pm_result {
            Ok(true) => {
                // legal request
                let post_message = GPTPostMessage::new_with_history(
                    self.chat_manager.get_histories(group_id, user_id).unwrap().clone(),
                );
                let return_message = match self.post_gpt_request(post_message).await {
                    Ok(gpt_recv_msg) => {
                        let msg = gpt_recv_msg.get_return_msg().unwrap();
                        self.chat_manager.push_history(group_id, user_id, msg.clone());
                        msg.into_contnet()
                    }
                    Err(e) => {
                        // todo might need to noted user to reset || clean
                        self.chat_manager.pop_history(group_id, user_id);
                        e.to_string()
                    }
                };
                self.chat_manager.after_handle_private_message(group_id, user_id);
                Some(SendMsg::new_message(group_id, user_id, return_message))
            }
            Ok(false) => None, // illegal request
            Err(e) => {
                self.chat_manager.after_handle_private_message(group_id, user_id);
                Some(SendMsg::new_message(group_id, user_id, e.to_string()))
            }
        }
    }

    async fn post_gpt_request(&self, post_message: GPTPostMessage) -> Result<GPTRecvMessage, error::BotError> {
        let body = serde_json::to_string(&post_message).unwrap();
        println!("body:{}", body);

        let c = match self.config.proxy_addr() {
            Some(addr) if !addr.is_empty() => {
                let proxy = reqwest::Proxy::https(addr)?;
                reqwest::Client::builder().proxy(proxy).build()?
            }
            _ => reqwest::Client::new(),
        };

        let resp_text = c
            .post(OPENAIAPIURL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key()))
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        let gpt_recv_msg = serde_json::from_str::<GPTRecvMessage>(&resp_text).map_err(|e| {
            println!("DEBUG: {:?}", resp_text);

            // try get GPT error message
            lazy_static::lazy_static! {
                static ref RE: regex::Regex = regex::Regex::new(
                    r#"message": "(.*?)""#,
                ).unwrap();
            }
            if let Some(str) = RE.find(&resp_text) {
                error::BotError::GPTReturnError(str.as_str().into())
            } else {
                error::BotError::GPTReturnError(e.to_string())
            }
        })?;

        Ok(gpt_recv_msg)
    }
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum BotError {
        #[error("error:HttpsRequestError {0}")]
        HttpsRequestError(String),

        #[error("error:GPTReturnError {0}")]
        GPTReturnError(String),
    }

    impl From<reqwest::Error> for BotError {
        fn from(value: reqwest::Error) -> Self {
            BotError::HttpsRequestError(value.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    async fn do_test_send_req() {
        let config = Config::test_new();
        let bot = Bot::new(config).await;
        let return_message = bot
            .post_gpt_request(GPTPostMessage::new_basic_message("hello".into()))
            .await;
        println!("return_message:{:?}", return_message);
    }

    #[test]
    fn test_send_req() {
        tokio_test::block_on(do_test_send_req())
    }
}
