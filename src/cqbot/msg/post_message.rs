use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::PostMessageType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Group,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sender {
    age: i64,
    nickname: String,
    sex: String,
    user_id: u64,
}

/// Type: Message,
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMessageMsg {
    post_type: PostMessageType, // PostMessageType::Message

    message_type: MessageType, // group message or private message

    time: u64,    // timestamp
    self_id: u64, // self qq number
    user_id: u64, // sender qq number

    sub_type: String, // normal ? friend ?
    sender: Sender,
    message: String,
    raw_message: String,
    message_id: i64,
    font: i64,

    // for group message
    group_id: Option<u64>,

    // for private message
    target_id: Option<u64>, // self qq number the same
}

impl PostMessageMsg {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    /// return (if is a at msg, clean msg field's cq code)
    pub fn pre_parse_msg(mut self, bot_id: u64) -> (bool, Self) {
        // lazy_static! {
        //     static ref AT_MSG_RE: Regex = Regex::new(r#"CQ:at,qq=[0-9]*"#).unwrap();
        // }
        let at_msg_re = Regex::new(format!("CQ:at,qq={}", bot_id).as_str()).unwrap();
        let mut is_at_msg = false;
        if at_msg_re.is_match(&self.raw_message) {
            is_at_msg = true;
        }

        lazy_static! {
            static ref CQ_CODE_RE: Regex = Regex::new(r#"\[CQ:.*?\]"#).unwrap();
        }
        self.message = CQ_CODE_RE.replace_all(&self.message, "").trim().to_string();

        (is_at_msg, self)
    }
    pub fn group_id(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn message_ts(&self) -> u64 {
        self.time
    }
}
