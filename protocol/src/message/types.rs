use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

/// 消息类型枚举，对应不同的协议操作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 生产消息请求
    Produce = 0,
    /// 获取消息请求
    Fetch = 1,
    /// 获取元数据请求
    Metadata = 2,
    /// 获取偏移量请求
    OffsetFetch = 3,
    /// 加入消费者组请求
    JoinGroup = 4,
    /// 同步消费者组状态请求
    SyncGroup = 5,
    /// 创建主题请求
    CreateTopic = 6,
    /// 删除主题请求
    DeleteTopic = 7,
    /// 描述主题请求
    DescribeTopic = 8,
    /// 列出主题请求
    ListTopics = 9,
    /// 更新主题配置请求
    UpdateTopicConfig = 10,
    /// 获取集群信息请求
    GetClusterInfo = 11,
    /// 心跳请求
    Heartbeat = 12,
    /// 离开消费者组请求
    LeaveGroup = 13,
    /// 未知消息类型
    Unknown = 255,
}

impl Hash for MessageType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Produce,
            1 => MessageType::Fetch,
            2 => MessageType::Metadata,
            3 => MessageType::OffsetFetch,
            4 => MessageType::JoinGroup,
            5 => MessageType::SyncGroup,
            6 => MessageType::CreateTopic,
            7 => MessageType::DeleteTopic,
            8 => MessageType::DescribeTopic,
            9 => MessageType::ListTopics,
            10 => MessageType::UpdateTopicConfig,
            11 => MessageType::GetClusterInfo,
            12 => MessageType::Heartbeat,
            13 => MessageType::LeaveGroup,
            _ => MessageType::Unknown,
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        value as u8
    }
} 