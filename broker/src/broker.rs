use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::metadata::{TopicMetadata, MetadataManager, TopicConfig, PartitionMetadata};
use crate::topic::Topic;
use protocol::{ClientRequest, ProduceRequest, FetchRequest, MetadataRequest, OffsetFetchRequest, JoinGroupRequest, SyncGroupRequest};

/// Broker 是 Kafka 的核心组件，负责管理主题、处理消息和客户端请求
pub struct Broker {
    /// 存储所有主题的映射表，使用 Arc<Mutex> 实现线程安全
    topics: Arc<Mutex<HashMap<String, Topic>>>,
    /// 管理主题元数据的组件
    metadata_manager: Arc<MetadataManager>,
    /// 存储消费者组的偏移量信息，格式为: group_id -> (topic-partition -> offset)
    offsets: Arc<Mutex<HashMap<String, HashMap<String, u32>>>>,
}

impl Broker {
    /// 创建一个新的 Broker 实例
    pub fn new() -> Self {
        Broker {
            topics: Arc::new(Mutex::new(HashMap::new())),
            metadata_manager: Arc::new(MetadataManager::new()),
            offsets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 创建一个新的主题
    /// 
    /// # Arguments
    /// * `topic` - 主题名称
    /// * `partitions` - 分区数量
    /// 
    /// # Returns
    /// * `Result<(), String>` - 创建成功返回 Ok(()), 失败返回错误信息
    pub fn create_topic(&self, topic: &str, partitions: usize) -> Result<(), String> {
        let config = TopicConfig {
            name: topic.to_string(),
            partitions,
            replication_factor: 1, // 默认值
            segment_size: 1024 * 1024, // 默认 1MB
        };

        let topic_metadata = TopicMetadata::new(topic.to_string(), config.clone());
        self.metadata_manager.add_topic(topic_metadata)?;

        let mut topic = Topic::new(topic.to_string(), config);
        for i in 0..partitions {
            let metadata = PartitionMetadata {
                id: i,
                leader: 1, // 默认值
                replicas: vec![1], // 默认值
                isr: vec![1], // 默认值
            };
            topic.create_partition(i, metadata)?;
        }

        let mut topics = self.topics.lock().map_err(|e| e.to_string())?;
        topics.insert(topic.to_string(), topic);
        Ok(())
    }

    /// 发送消息到指定主题
    /// 
    /// # Arguments
    /// * `topic` - 目标主题
    /// * `message` - 消息内容
    /// 
    /// # Returns
    /// * `Result<u64, String>` - 成功返回消息的偏移量，失败返回错误信息
    pub fn send_message(&self, topic: &str, message: Vec<u8>) -> Result<u64, String> {
        let topics = self.topics.lock().map_err(|e| e.to_string())?;
        let topic = topics.get(topic)
            .ok_or_else(|| "Topic not found".to_string())?;
        
        let partition_id = message.len() % topic.get_partition_count();
        topic.append_message(partition_id, message)
    }

    /// 从指定主题的分区获取消息
    /// 
    /// # Arguments
    /// * `topic` - 主题名称
    /// * `partition` - 分区 ID
    /// * `offset` - 消息偏移量
    /// 
    /// # Returns
    /// * `Result<Option<Vec<u8>>, String>` - 成功返回消息内容，失败返回错误信息
    pub fn fetch_message(&self, topic: &str, partition: usize, offset: u32) -> Result<Option<Vec<u8>>, String> {
        let topics = self.topics.lock().map_err(|e| e.to_string())?;
        let topic = topics.get(topic)
            .ok_or_else(|| "Topic not found".to_string())?;
        
        topic.read_message(partition, offset as u64)
    }

    /// 提交消费者组的偏移量
    /// 
    /// # Arguments
    /// * `group` - 消费者组 ID
    /// * `topic` - 主题名称
    /// * `partition` - 分区 ID
    /// * `offset` - 偏移量
    /// 
    /// # Returns
    /// * `Result<(), String>` - 成功返回 Ok(()), 失败返回错误信息
    pub fn commit_offset(&self, group: &str, topic: &str, partition: usize, offset: u32) -> Result<(), String> {
        let mut offsets = self.offsets.lock().map_err(|e| e.to_string())?;
        let group_offsets = offsets.entry(group.to_string()).or_insert(HashMap::new());
        group_offsets.insert(format!("{topic}-{partition}"), offset);
        Ok(())
    }

    /// 获取消费者组的偏移量
    /// 
    /// # Arguments
    /// * `group` - 消费者组 ID
    /// * `topic` - 主题名称
    /// * `partition` - 分区 ID
    /// 
    /// # Returns
    /// * `Result<Option<u32>, String>` - 成功返回偏移量，失败返回错误信息
    pub fn get_offset(&self, group: &str, topic: &str, partition: usize) -> Result<Option<u32>, String> {
        let offsets = self.offsets.lock().map_err(|e| e.to_string())?;
        Ok(offsets.get(group)
            .and_then(|group_offsets| group_offsets.get(&format!("{topic}-{partition}")))
            .copied())
    }

    // 内部方法
    /// 处理生产者请求
    fn handle_produce_request(&self, req: ProduceRequest) -> Result<(), String> {
        self.send_message(&req.topic, req.message)
            .map(|_| ())
    }

    /// 处理消费者请求
    fn handle_fetch_request(&self, req: FetchRequest) -> Result<(), String> {
        self.fetch_message(&req.topic, req.partition, req.offset)
            .map(|_| ())
    }

    /// 处理元数据请求
    fn handle_metadata_request(&self, _req: MetadataRequest) -> Result<(), String> {
        // TODO: 实现元数据请求处理
        Ok(())
    }

    /// 处理偏移量获取请求
    fn handle_offset_fetch_request(&self, req: OffsetFetchRequest) -> Result<(), String> {
        self.get_offset(&req.group_id, &req.topic, req.partition)
            .map(|_| ())
    }

    /// 处理加入消费者组请求
    fn handle_join_group_request(&self, _req: JoinGroupRequest) -> Result<(), String> {
        // TODO: 实现加入消费者组请求处理
        Ok(())
    }

    /// 处理同步消费者组请求
    fn handle_sync_group_request(&self, _req: SyncGroupRequest) -> Result<(), String> {
        // TODO: 实现同步消费者组请求处理
        Ok(())
    }

    /// 处理客户端请求的主入口
    /// 
    /// # Arguments
    /// * `request` - 客户端请求
    /// 
    /// # Returns
    /// * `Result<(), String>` - 处理成功返回 Ok(()), 失败返回错误信息
    pub fn handle_request(&self, request: ClientRequest) -> Result<(), String> {
        match request {
            ClientRequest::Produce(req) => self.handle_produce_request(req),
            ClientRequest::Fetch(req) => self.handle_fetch_request(req),
            ClientRequest::Metadata(req) => self.handle_metadata_request(req),
            ClientRequest::OffsetFetch(req) => self.handle_offset_fetch_request(req),
            ClientRequest::JoinGroup(req) => self.handle_join_group_request(req),
            ClientRequest::SyncGroup(req) => self.handle_sync_group_request(req),
        }
    }
}
