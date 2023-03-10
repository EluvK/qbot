use lazy_static::lazy_static;
use regex::Regex;
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
    pub fn new_private_msg(user_id: u64, message: String) -> Self {
        SendMsg {
            action: SendMsgAction::SendPrivateMsg,
            params: SendMsgParams {
                user_id: Some(user_id),
                group_id: None,
                message,
            },
        }
    }

    pub fn new_group_reply_msg_at(group_id: u64, user_id: u64, message: String) -> Self {
        SendMsg {
            action: SendMsgAction::SendGroupMsg,
            params: SendMsgParams {
                user_id: None,
                group_id: Some(group_id),
                message: format!("[CQ:at,qq={}]{}", user_id, message),
            },
        }
    }

    // pub fn new_group_reply_msg(group_id: u64, message_id: i64, message: String) -> Self {
    //     SendMsg {
    //         action: SendMsgAction::SendGroupMsg,
    //         params: SendMsgParams {
    //             user_id: None,
    //             group_id: Some(group_id),
    //             message: format!("[CQ:reply,id={}]{}", message_id, message),
    //         },
    //     }
    // }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PostMessageType {
    Message,
    MetaEvent,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RecvMsg {
    post_type: PostMessageType,
    message_type: MessageType, // group message or private message
    time: u64,                 // timestamp
    self_id: u64,              // self qq number
    sub_type: String,          // normal ? friend ?

    user_id: u64, // sender qq number
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

impl RecvMsg {
    // pub fn is_group_message(&self) -> bool {
    //     self.message_type == MessageType::Group
    // }
    // pub fn is_private_message(&self) -> bool {
    //     self.message_type == MessageType::Private
    // }

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
    #[allow(dead_code)]
    pub fn message_id(&self) -> i64 {
        self.message_id
    }
    pub fn message(&self) -> &String {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_recvmsg_serde() {
        let msg_str = r#"{"post_type":"message","message_type":"private","time":1678331835,"self_id":222,"sub_type":"friend","sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":1996520388,"user_id":999,"target_id":222,"message":"test","raw_message":"test","font":0}"#;
        let recv_msg = serde_json::from_str::<RecvMsg>(msg_str).unwrap();
        println!("{:?}", recv_msg);

        let msg_str_2 = r#"{"post_type":"message","message_type":"group","time":1678331780,"self_id":222,"sub_type":"normal","group_id":777,"message":"test","raw_message":"test","sender":{"age":0,"area":"","card":"","level":"","nickname":"Mr.Eucalypt","role":"member","sex":"unknown","title":"","user_id":999},"user_id":999,"message_id":-645121622,"anonymous":null,"font":0,"message_seq":133}"#;
        let recv_msg_2 = serde_json::from_str::<RecvMsg>(msg_str_2).unwrap();
        println!("{:?}", recv_msg_2);

        // let _msg_str_3 = r#"{"post_type":"meta_event","meta_event_type":"heartbeat","time":1678331981,"self_id":222,"status":{"app_enabled":true,"app_good":true,"app_initialized":true,"good":true,"online":true,"plugins_good":null,"stat":{"packet_received":201,"packet_sent":188,"packet_lost":0,"message_received":3,"message_sent":0,"disconnect_times":0,"lost_times":0,"last_message_time":1678331835}},"interval":5000}"#;
        // let recv_msg_3 = serde_json::from_str::<RecvMsg>(_msg_str_3).unwrap();
        // println!("{:?}", recv_msg_3);
    }

    #[test]
    fn test_recv_at() {
        let msg_str = r#"{"post_type":"message","message_type":"group","time":1678336087,"self_id":222,"sub_type":"normal","anonymous":null,"group_id":777,"raw_message":"[CQ:at,qq=222] 1","sender":{"age":0,"area":"","card":"","level":"","nickname":"Mr.Eucalypt","role":"member","sex":"unknown","title":"","user_id":999},"user_id":999,"message_id":1206430729,"font":0,"message":"[CQ:at,qq=222] 1","message_seq":134}"#;
        let recv_msg = serde_json::from_str::<RecvMsg>(msg_str).unwrap();
        println!("{:?}", recv_msg);
        println!("{:?}", recv_msg.pre_parse_msg(222));

        let msg_str_2 = r#"{"post_type":"message","message_type":"private","time":1678336074,"self_id":222,"sub_type":"friend","target_id":222,"message":"柠檬茶 1","raw_message":"柠檬茶 1","font":0,"sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":-111094286,"user_id":999}"#;
        let recv_msg_2 = serde_json::from_str::<RecvMsg>(msg_str_2).unwrap();
        println!("{:?}", recv_msg_2);
        println!("{:?}", recv_msg_2.pre_parse_msg(222));

        let msg_str_3 = r#"{"post_type":"message","message_type":"private","time":1678334227,"self_id":222,"sub_type":"friend","sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":67453260,"user_id":999,"target_id":222,"message":"[CQ:image,file=7ecb49dcfcdcca29feb728cf8811bb37.image,url=https://c2cpicdw.qpic.cn/offpic_new/999//999-3676750358-7ECB49DCFCDCCA29FEB728CF8811BB37/0?term=2\u0026amp;is_origin=0]","raw_message":"[CQ:image,file=7ecb49dcfcdcca29feb728cf8811bb37.image,url=https://c2cpicdw.qpic.cn/offpic_new/999//999-3676750358-7ECB49DCFCDCCA29FEB728CF8811BB37/0?term=2\u0026amp;is_origin=0]","font":0}"#;
        let recv_msg_3 = serde_json::from_str::<RecvMsg>(msg_str_3).unwrap();
        println!("{:?}", recv_msg_3);
        println!("{:?}", recv_msg_3.pre_parse_msg(222));
    }
}
