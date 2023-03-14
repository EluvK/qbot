use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SendMsgAction {
    SendPrivateMsg,
    SendGroupMsg,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendMsgParams {
    user_id: Option<u64>,
    group_id: Option<u64>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMsg {
    action: SendMsgAction,
    params: SendMsgParams,
}

impl SendMsg {
    fn new_private_msg(user_id: u64, message: String) -> Self {
        SendMsg {
            action: SendMsgAction::SendPrivateMsg,
            params: SendMsgParams {
                user_id: Some(user_id),
                group_id: None,
                message,
            },
        }
    }

    fn new_group_reply_msg_at(group_id: u64, user_id: u64, message: String) -> Self {
        SendMsg {
            action: SendMsgAction::SendGroupMsg,
            params: SendMsgParams {
                user_id: None,
                group_id: Some(group_id),
                message: format!("[CQ:at,qq={}]{}", user_id, message),
            },
        }
    }

    pub fn new_message(group_id: Option<u64>, user_id: u64, message: String) -> Self {
        match group_id {
            Some(group_id) => Self::new_group_reply_msg_at(group_id, user_id, message),
            None => Self::new_private_msg(user_id, message),
        }
    }
}
