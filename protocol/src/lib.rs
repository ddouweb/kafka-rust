use serde::{Serialize, Deserialize};

pub mod message;
pub use message::{BinaryMessage, MessageType};

/// 消息处理器trait
pub trait MessageHandler: Send + Sync {
    fn handle_message(&self, message: BinaryMessage) -> Option<message::BinaryMessage>;
}

/// 表示对 Kafka 协议的客户端请求。
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientRequest {
    /// 向特定主题和分区发送消息的请求。
    Produce(ProduceRequest),
    /// 从特定主题和分区获取消息的请求。
    Fetch(FetchRequest),
    /// 获取主题元数据的请求。
    Metadata(MetadataRequest),
    /// 获取特定消费者组偏移量的请求。
    OffsetFetch(OffsetFetchRequest),
    /// 加入消费者组的请求。
    JoinGroup(JoinGroupRequest),
    /// 同步消费者组状态的请求。
    SyncGroup(SyncGroupRequest),
    /// 创建主题的请求。
    CreateTopic(CreateTopicRequest),
    /// 删除主题的请求。
    DeleteTopic(DeleteTopicRequest),
    /// 获取主题描述的请求。
    DescribeTopic(DescribeTopicRequest),
    /// 列出所有主题的请求。
    ListTopics(ListTopicsRequest),
    /// 更新主题配置的请求。
    UpdateTopicConfig(UpdateTopicConfigRequest),
    /// 获取集群信息的请求。
    GetClusterInfo(GetClusterInfoRequest),
}

/// 表示向 Kafka 主题发送消息的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct ProduceRequest {
    /// 要发送消息的主题名称。
    pub topic: String,
    /// 要发送消息的主题分区。
    pub partition: usize,
    /// 要发送的消息内容。
    pub message: Vec<u8>,
}

/// 表示从 Kafka 主题获取消息的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchRequest {
    /// 要获取消息的主题名称。
    pub topic: String,
    /// 要获取消息的主题分区。
    pub partition: usize,
    /// 开始获取消息的偏移量。
    pub offset: u32,
}

/// 表示获取 Kafka 主题元数据的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataRequest {
    /// 要获取元数据的主题名称。如果为 `None`，则获取所有主题的元数据。
    pub topic: Option<String>,
}

/// 表示获取特定消费者组偏移量的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetFetchRequest {
    /// 消费者组的 ID。
    pub group_id: String,
    /// 要获取偏移量的主题名称。
    pub topic: String,
    /// 要获取偏移量的主题分区。
    pub partition: usize,
}

/// 表示加入 Kafka 消费者组的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    /// 要加入的消费者组 ID。
    pub group_id: String,
    /// 尝试加入组的消费者 ID。
    pub consumer_id: String,
}

/// 表示同步 Kafka 消费者组状态的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncGroupRequest {
    /// 要同步的消费者组 ID。
    pub group_id: String,
    /// 参与同步的消费者 ID。
    pub consumer_id: String,
    /// 组的主题-分区分配。
    pub assignments: Vec<(String, usize)>,
}

/// 表示创建主题的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    /// 主题名称
    pub name: String,
    /// 分区数量
    pub num_partitions: usize,
    /// 副本因子
    pub replication_factor: usize,
    /// 其他配置项
    pub configs: std::collections::HashMap<String, String>,
}

/// 表示删除主题的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTopicRequest {
    /// 主题名称
    pub name: String,
}

/// 表示获取主题描述的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeTopicRequest {
    /// 主题名称
    pub name: String,
}

/// 表示列出所有主题的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTopicsRequest {}

/// 表示更新主题配置的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTopicConfigRequest {
    /// 主题名称
    pub name: String,
    /// 新的配置项
    pub configs: std::collections::HashMap<String, String>,
}

/// 表示获取集群信息的请求。
#[derive(Debug, Serialize, Deserialize)]
pub struct GetClusterInfoRequest {
    // 空结构体，表示不需要任何参数
}