use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::PostMessageType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Friend,
    Group,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RequestType::Friend => "friend",
            RequestType::Group => "group",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequestMsg {
    post_type: PostMessageType, // PostMessageType::Request

    request_type: RequestType, // friend or group request

    time: u64,    // timestamp
    self_id: u64, // self qq number
    user_id: u64, // sender qq number

    comment: String,
    flag: String,
}

impl Display for PostRequestMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type: {}, user_id: {}, comment:{}",
            self.request_type, self.user_id, self.comment
        )
    }
}
