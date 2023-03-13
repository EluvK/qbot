use std::{fmt::Display, marker::PhantomData};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostMessageType {
    Message,
    MetaEvent,
    Request,
    Notice,
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

/// Used to check PostType.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMsg {
    pub post_type: PostMessageType,
    #[serde(skip)]
    _others: PhantomData<PostMsg>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetaEventMsg {
    // todo, now really not need these
}

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

/// https://docs.go-cqhttp.org/reference/data_struct.html#post-notice-type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NoticeType {
    GroupUpload,
    GroupAdmin,
    GroupDecrease,
    GroupIncrease,
    GroupBan,
    FriendAdd, // only one needed for now
    GroupRecall,
    FriendRecall,
    GroupCard,
    OfflineFile,
    ClientStatus,
    Essence,
    Notify,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostNoticeMsg {
    post_type: PostMessageType, // PostMessageType::Notice

    pub notice_type: NoticeType,

    time: u64,        // timestamp
    self_id: u64,     // self qq number
    pub user_id: u64, // sender qq number
}

impl PostMessageMsg {
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
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn message_ts(&self) -> u64 {
        self.time
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_recvmsg_serde() {
        let msg_str = r#"{"post_type":"message","message_type":"private","time":1678331835,"self_id":222,"sub_type":"friend","sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":1996520388,"user_id":999,"target_id":222,"message":"test","raw_message":"test","font":0}"#;
        let recv_msg = serde_json::from_str::<PostMessageMsg>(msg_str).unwrap();
        println!("{:?}", recv_msg);

        let msg_str_2 = r#"{"post_type":"message","message_type":"group","time":1678331780,"self_id":222,"sub_type":"normal","group_id":777,"message":"test","raw_message":"test","sender":{"age":0,"area":"","card":"","level":"","nickname":"Mr.Eucalypt","role":"member","sex":"unknown","title":"","user_id":999},"user_id":999,"message_id":-645121622,"anonymous":null,"font":0,"message_seq":133}"#;
        let recv_msg_2 = serde_json::from_str::<PostMessageMsg>(msg_str_2).unwrap();
        println!("{:?}", recv_msg_2);

        // let _msg_str_3 = r#"{"post_type":"meta_event","meta_event_type":"heartbeat","time":1678331981,"self_id":222,"status":{"app_enabled":true,"app_good":true,"app_initialized":true,"good":true,"online":true,"plugins_good":null,"stat":{"packet_received":201,"packet_sent":188,"packet_lost":0,"message_received":3,"message_sent":0,"disconnect_times":0,"lost_times":0,"last_message_time":1678331835}},"interval":5000}"#;
        // let recv_msg_3 = serde_json::from_str::<RecvMsg>(_msg_str_3).unwrap();
        // println!("{:?}", recv_msg_3);
    }

    #[test]
    fn test_recv_at() {
        let msg_str = r#"{"post_type":"message","message_type":"group","time":1678336087,"self_id":222,"sub_type":"normal","anonymous":null,"group_id":777,"raw_message":"[CQ:at,qq=222] 1","sender":{"age":0,"area":"","card":"","level":"","nickname":"Mr.Eucalypt","role":"member","sex":"unknown","title":"","user_id":999},"user_id":999,"message_id":1206430729,"font":0,"message":"[CQ:at,qq=222] 1","message_seq":134}"#;
        let recv_msg = serde_json::from_str::<PostMessageMsg>(msg_str).unwrap();
        println!("{:?}", recv_msg);
        println!("{:?}", recv_msg.pre_parse_msg(222));

        let msg_str_2 = r#"{"post_type":"message","message_type":"private","time":1678336074,"self_id":222,"sub_type":"friend","target_id":222,"message":"柠檬茶 1","raw_message":"柠檬茶 1","font":0,"sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":-111094286,"user_id":999}"#;
        let recv_msg_2 = serde_json::from_str::<PostMessageMsg>(msg_str_2).unwrap();
        println!("{:?}", recv_msg_2);
        println!("{:?}", recv_msg_2.pre_parse_msg(222));

        let msg_str_3 = r#"{"post_type":"message","message_type":"private","time":1678334227,"self_id":222,"sub_type":"friend","sender":{"age":0,"nickname":"Mr.Eucalypt","sex":"unknown","user_id":999},"message_id":67453260,"user_id":999,"target_id":222,"message":"[CQ:image,file=7ecb49dcfcdcca29feb728cf8811bb37.image,url=https://c2cpicdw.qpic.cn/offpic_new/999//999-3676750358-7ECB49DCFCDCCA29FEB728CF8811BB37/0?term=2\u0026amp;is_origin=0]","raw_message":"[CQ:image,file=7ecb49dcfcdcca29feb728cf8811bb37.image,url=https://c2cpicdw.qpic.cn/offpic_new/999//999-3676750358-7ECB49DCFCDCCA29FEB728CF8811BB37/0?term=2\u0026amp;is_origin=0]","font":0}"#;
        let recv_msg_3 = serde_json::from_str::<PostMessageMsg>(msg_str_3).unwrap();
        println!("{:?}", recv_msg_3);
        println!("{:?}", recv_msg_3.pre_parse_msg(222));
    }

    #[test]
    fn test_notice_request_msg() {
        let msg_str = r#"{"post_type":"notice","notice_type":"friend_add","time":1678697481,"self_id":2221488475,"user_id":948211941}"#;
        let recv_msg = serde_json::from_str::<PostNoticeMsg>(msg_str).unwrap();
        println!("{:?}", recv_msg);
        let msg_str_2 = r#"{"post_type":"request","request_type":"friend","time":1678697473,"self_id":2221488475,"user_id":948211941,"comment":"","flag":"1678697473000000"}"#;
        let recv_msg_2 = serde_json::from_str::<PostRequestMsg>(msg_str_2).unwrap();
        println!("{:?}", recv_msg_2);
    }
}
