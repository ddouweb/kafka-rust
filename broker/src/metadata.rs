use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 分区元数据，包含分区的配置信息
#[derive(Debug, Clone)]
pub struct PartitionMetadata {
    /// 分区 ID
    pub id: usize,
    /// 分区的领导者 broker ID
    pub leader: u32,
    /// 分区的副本列表
    pub replicas: Vec<u32>,
    /// 同步副本列表 (In-Sync Replicas)
    pub isr: Vec<u32>,
}

/// 主题元数据，包含主题的配置和分区信息
#[derive(Debug, Clone)]
pub struct TopicMetadata {
    /// 主题名称
    pub name: String,
    /// 主题的分区列表
    pub partitions: Vec<PartitionMetadata>,
    /// 主题的配置信息
    pub config: TopicConfig,
}

/// 主题配置，定义主题的基本属性
#[derive(Debug, Clone)]
pub struct TopicConfig {
    /// 主题名称
    pub name: String,
    /// 分区数量
    pub partitions: usize,
    /// 副本因子（每个分区的副本数）
    pub replication_factor: usize,
    /// 单个日志段的最大大小（字节）
    pub segment_size: usize,
    /// 基础目录
    pub base_dir: String,
}

impl TopicMetadata {
    /// 创建新的主题元数据
    /// 
    /// # Arguments
    /// * `name` - 主题名称
    /// * `config` - 主题配置
    pub fn new(name: String, config: TopicConfig) -> Self {
        Self {
            name,
            partitions: Vec::new(),
            config,
        }
    }

    /// 添加分区到主题
    /// 
    /// # Arguments
    /// * `partition` - 分区元数据
    pub fn add_partition(&mut self, partition: PartitionMetadata) {
        self.partitions.push(partition);
    }

    /// 从主题中删除指定分区
    /// 
    /// # Arguments
    /// * `partition_id` - 要删除的分区 ID
    pub fn remove_partition(&mut self, partition_id: usize) {
        self.partitions.retain(|p| p.id != partition_id);
    }

    /// 获取指定 ID 的分区元数据
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// 
    /// # Returns
    /// * `Option<&PartitionMetadata>` - 如果找到分区则返回其元数据，否则返回 None
    pub fn get_partition(&self, partition_id: usize) -> Option<&PartitionMetadata> {
        self.partitions.iter().find(|p| p.id == partition_id)
    }
}

/// 元数据管理器，负责管理所有主题的元数据
#[derive(Debug)]
pub struct MetadataManager {
    /// 主题元数据映射表，使用 Arc<Mutex> 实现线程安全
    topics: Arc<Mutex<HashMap<String, TopicMetadata>>>,
}

impl MetadataManager {
    /// 创建新的元数据管理器
    pub fn new() -> Self {
        Self {
            topics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 添加新主题到元数据管理器
    /// 
    /// # Arguments
    /// * `topic` - 主题元数据
    /// 
    /// # Returns
    /// * `Result<(), String>` - 添加成功返回 Ok(()), 失败返回错误信息
    pub fn add_topic(&self, topic: TopicMetadata) -> Result<(), String> {
        let mut topics = self.topics.lock().map_err(|e| e.to_string())?;
        if topics.contains_key(&topic.name) {
            return Err("Topic already exists".to_string());
        }
        topics.insert(topic.name.clone(), topic);
        Ok(())
    }

    /// 获取指定名称的主题元数据
    /// 
    /// # Arguments
    /// * `name` - 主题名称
    /// 
    /// # Returns
    /// * `Result<Option<TopicMetadata>, String>` - 成功返回主题元数据，失败返回错误信息
    #[allow(dead_code)]
    pub fn get_topic(&self, name: &str) -> Result<Option<TopicMetadata>, String> {
        let topics = self.topics.lock().map_err(|e| e.to_string())?;
        Ok(topics.get(name).cloned())
    }

    /// 从元数据管理器中删除指定主题
    /// 
    /// # Arguments
    /// * `name` - 要删除的主题名称
    /// 
    /// # Returns
    /// * `Result<(), String>` - 删除成功返回 Ok(()), 失败返回错误信息
    #[allow(dead_code)]
    pub fn remove_topic(&self, name: &str) -> Result<(), String> {
        let mut topics = self.topics.lock().map_err(|e| e.to_string())?;
        topics.remove(name);
        Ok(())
    }
}
