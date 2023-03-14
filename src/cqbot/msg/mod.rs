use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

mod send_msg;
pub use send_msg::SendMsg;

mod post_message;
mod post_meta_event;
mod post_notice;
mod post_request;
pub use post_message::{MessageType, PostMessageMsg};
pub use post_meta_event::PostMetaEventMsg;
pub use post_notice::{NoticeType, PostNoticeMsg};
pub use post_request::{PostRequestMsg, RequestType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostMessageType {
    Message,
    MetaEvent,
    Request,
    Notice,
}

/// Used to check PostType.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMsg {
    pub post_type: PostMessageType,
    #[serde(skip)]
    _others: PhantomData<PostMsg>,
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
