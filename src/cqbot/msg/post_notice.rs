use serde::{Deserialize, Serialize};

use super::PostMessageType;

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
