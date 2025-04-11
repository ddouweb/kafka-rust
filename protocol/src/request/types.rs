use serde::{Serialize, Deserialize};

/// 客户端请求类型
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 生产消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProduceRequest {
    pub topic: String,
    pub partition: i32,
    pub messages: Vec<Vec<u8>>,
}

/// 获取消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchRequest {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub max_bytes: i32,
}

/// 获取元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataRequest {
    pub topics: Vec<String>,
}

/// 获取偏移量请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetFetchRequest {
    pub group_id: String,
    pub topics: Vec<String>,
}

/// 加入消费者组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    pub group_id: String,
    pub member_id: String,
    pub protocol_type: String,
}

/// 同步消费者组状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncGroupRequest {
    pub group_id: String,
    pub member_id: String,
    pub generation_id: i32,
}

/// 心跳请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub group_id: String,
    pub member_id: String,
    pub generation_id: i32,
}

/// 离开消费者组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveGroupRequest {
    pub group_id: String,
    pub member_id: String,
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