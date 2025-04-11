use serde::{Serialize, Deserialize};

/// 服务器响应类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerResponse {
    /// 生产消息响应
    Produce(ProduceResponse),
    /// 获取消息响应
    Fetch(FetchResponse),
    /// 获取元数据响应
    Metadata(MetadataResponse),
    /// 获取偏移量响应
    OffsetFetch(OffsetFetchResponse),
    /// 加入消费者组响应
    JoinGroup(JoinGroupResponse),
    /// 同步消费者组状态响应
    SyncGroup(SyncGroupResponse),
    /// 心跳响应
    Heartbeat(HeartbeatResponse),
    /// 离开消费者组响应
    LeaveGroup(LeaveGroupResponse),
}

/// 生产消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProduceResponse {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub error_code: i16,
}

/// 获取消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchResponse {
    pub topic: String,
    pub partition: i32,
    pub error_code: i16,
    pub high_watermark: i64,
    pub messages: Vec<Vec<u8>>,
}

/// 获取元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub brokers: Vec<Broker>,
    pub topics: Vec<TopicMetadata>,
}

/// 获取偏移量响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetFetchResponse {
    pub group_id: String,
    pub topics: Vec<TopicOffset>,
}

/// 加入消费者组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroupResponse {
    pub group_id: String,
    pub member_id: String,
    pub leader_id: String,
    pub error_code: i16,
}

/// 同步消费者组状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncGroupResponse {
    pub group_id: String,
    pub member_id: String,
    pub error_code: i16,
}

/// 心跳响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub group_id: String,
    pub member_id: String,
    pub error_code: i16,
}

/// 离开消费者组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveGroupResponse {
    pub group_id: String,
    pub member_id: String,
    pub error_code: i16,
}

/// 代理节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}

/// 主题元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMetadata {
    pub topic: String,
    pub partitions: Vec<PartitionMetadata>,
}

/// 分区元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionMetadata {
    pub partition: i32,
    pub leader: i32,
    pub replicas: Vec<i32>,
    pub isr: Vec<i32>,
}

/// 主题偏移量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicOffset {
    pub topic: String,
    pub partitions: Vec<PartitionOffset>,
}

/// 分区偏移量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionOffset {
    pub partition: i32,
    pub offset: i64,
    pub error_code: i16,
} 