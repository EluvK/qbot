use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    cqbot::msg::{MessageType, RecvMsg, SendMsg},
    gpt::{GPTPostMessage, GPTRecvMessage},
    private_manager::{GroupManager, PrivateManager},
};

pub struct Bot {
    qq: u64,
    proxy_addr: Option<String>,
    api_key: String,
    web_socket_stream: Box<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    private_manager: PrivateManager,
    group_manager: GroupManager,
}

static OPENAIAPIURL: &str = "https://api.openai.com/v1/chat/completions";

impl Bot {
    pub async fn new(url: &str, proxy: Option<String>, api_key: String, qq: u64) -> Self {
        let (wss, _) = connect_async(url).await.expect("Fail to connect");
        Self {
            qq,
            proxy_addr: proxy,
            api_key,
            web_socket_stream: Box::new(wss),
            private_manager: PrivateManager::new(0),
            group_manager: GroupManager::new(),
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
            if let Some(send_msg) = self.handler_message(msg).await {
                self.send_back_message(send_msg).await
            }
        }
    }

    async fn handler_message(&mut self, message: Message) -> Option<SendMsg> {
        if let Message::Text(text) = message {
            // println!("Handle Text:{}", &text);
            let msg = serde_json::from_str::<RecvMsg>(&text).ok()?;
            let (is_at, msg) = msg.pre_parse_msg(self.qq);
            println!("is_at:{:?}, RecvMessage:{:?}", is_at, &msg);
            match msg.message_type() {
                MessageType::Group => {
                    if is_at {
                        Some(self.handler_group_message(msg).await)
                    } else {
                        None
                    }
                }
                MessageType::Private => Some(self.handler_private_message(msg).await),
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

    async fn handler_group_message(&mut self, message: RecvMsg) -> SendMsg {
        let group_id = message.group_id();
        let user_id = message.user_id();
        let ts = message.message_ts();
        let gpm = self
            .group_manager
            .pre_handle_private_message(group_id, user_id, ts, message.message().clone());

        match gpm {
            Ok(_) => {
                // legal request
                let post_message = GPTPostMessage::new_with_history(
                    self.group_manager.get_histories(group_id, user_id).unwrap().clone(),
                );
                let return_message = match self.post_gpt_request(post_message).await {
                    Ok(gpt_recv_msg) => {
                        let msg = gpt_recv_msg.get_return_msg().unwrap();
                        self.group_manager.push_history(group_id, user_id, msg.clone());
                        msg.into_contnet()
                    }
                    Err(e) => {
                        // todo might need to noted user to reset || clean
                        self.group_manager.pop_history(group_id, user_id);
                        e.to_string()
                    }
                };
                self.group_manager.after_handle_private_message(group_id, user_id);
                SendMsg::new_group_reply_msg_at(group_id, user_id, return_message)
            }
            Err(e) => {
                self.group_manager.after_handle_private_message(group_id, user_id);
                SendMsg::new_group_reply_msg_at(group_id, user_id, e.to_string())
            }
        }

        // let post_message = GPTPostMessage::new_basic_message(message.message().clone());
        // let return_message = match self.post_gpt_request(post_message).await {
        //     Ok(gpt_recv_msg) => {
        //         let msg = gpt_recv_msg.get_return_msg().unwrap();
        //         msg.into_contnet()
        //     }
        //     Err(e) => e.to_string(),
        // };
        // SendMsg::new_group_reply_msg(message.group_id(), message.message_id(), return_message)
    }
    async fn handler_private_message(&mut self, message: RecvMsg) -> SendMsg {
        let user_id = message.user_id();
        let pm =
            self.private_manager
                .pre_handle_private_message(user_id, message.message_ts(), message.message().clone());
        match pm {
            Ok(_) => {
                // legal request
                let post_message =
                    GPTPostMessage::new_with_history(self.private_manager.get_histories(user_id).unwrap().clone());
                let return_message = match self.post_gpt_request(post_message).await {
                    Ok(gpt_recv_msg) => {
                        let msg = gpt_recv_msg.get_return_msg().unwrap();
                        self.private_manager.push_history(user_id, msg.clone());
                        msg.into_contnet()
                    }
                    Err(e) => {
                        // todo might need to noted user to reset || clean
                        self.private_manager.pop_history(user_id);
                        e.to_string()
                    }
                };
                self.private_manager.after_handle_private_message(user_id);
                SendMsg::new_private_msg(user_id, return_message)
            }
            Err(e) => {
                self.private_manager.after_handle_private_message(user_id);
                SendMsg::new_private_msg(user_id, e.to_string())
            }
        }
    }

    async fn post_gpt_request(&self, post_message: GPTPostMessage) -> Result<GPTRecvMessage, error::BotError> {
        let body = serde_json::to_string(&post_message).unwrap();
        println!("body:{}", body);

        // proxy:
        // let proxy = reqwest::Proxy::https("socks5h://127.0.0.1:1080")?;
        // let c = reqwest::Client::builder().proxy(proxy).build()?;
        // or ::
        // let c = reqwest::Client::new();
        let c = self.proxy_addr.clone().map_or(Ok(reqwest::Client::new()), |addr| {
            let proxy = reqwest::Proxy::https(addr.as_str())?;
            reqwest::Client::builder().proxy(proxy).build()
        })?;

        let resp_text = c
            .post(OPENAIAPIURL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", String::from(&self.api_key)))
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

        #[error("error:GPTReturnDeserError {0}")]
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
        let bot = Bot::new(
            "ws://localhost:8080/ws",
            Some("socks5h://127.0.0.1:1080".into()),
            "sk-xx".into(),
            222,
        )
        .await;
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
