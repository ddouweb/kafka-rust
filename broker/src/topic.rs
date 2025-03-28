use std::sync::{Arc, Mutex};
use queue::LogQueue;
use crate::metadata::{TopicConfig, PartitionMetadata};
use std::fmt;

/// 主题，代表一个消息主题，包含多个分区
#[derive(Debug)]
pub struct Topic {
    /// 主题名称
    name: String,
    /// 主题的分区列表
    partitions: Vec<Partition>,
    /// 主题的配置信息
    config: TopicConfig,
}

/// 分区，代表主题的一个分区，包含消息队列和元数据
#[derive(Debug)]
pub struct Partition {
    /// 分区 ID
    id: usize,
    /// 消息队列，使用 Arc<Mutex> 实现线程安全
    queue: Arc<Mutex<LogQueue>>,
    /// 分区元数据
    metadata: PartitionMetadata,
}

impl Topic {
    /// 创建新的主题
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

    /// 创建新的分区
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// * `metadata` - 分区元数据
    /// 
    /// # Returns
    /// * `Result<(), String>` - 创建成功返回 Ok(()), 失败返回错误信息
    pub fn create_partition(&mut self, partition_id: usize, metadata: PartitionMetadata) -> Result<(), String> {
        if self.partitions.iter().any(|p| p.id == partition_id) {
            return Err("Partition already exists".to_string());
        }

        let queue = Arc::new(Mutex::new(LogQueue::new(
            &format!("{}/partition-{}", self.name, partition_id),
            self.config.segment_size,
        ).unwrap()));

        self.partitions.push(Partition {
            id: partition_id,
            queue,
            metadata,
        });

        Ok(())
    }

    /// 删除指定分区
    /// 
    /// # Arguments
    /// * `partition_id` - 要删除的分区 ID
    /// 
    /// # Returns
    /// * `Result<(), String>` - 删除成功返回 Ok(()), 失败返回错误信息
    pub fn delete_partition(&mut self, partition_id: usize) -> Result<(), String> {
        if let Some(index) = self.partitions.iter().position(|p| p.id == partition_id) {
            self.partitions.remove(index);
            Ok(())
        } else {
            Err("Partition not found".to_string())
        }
    }

    /// 获取指定 ID 的分区
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// 
    /// # Returns
    /// * `Option<&Partition>` - 如果找到分区则返回其引用，否则返回 None
    pub fn get_partition(&self, partition_id: usize) -> Option<&Partition> {
        self.partitions.iter().find(|p| p.id == partition_id)
    }

    /// 获取指定 ID 的分区的可变引用
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// 
    /// # Returns
    /// * `Option<&mut Partition>` - 如果找到分区则返回其可变引用，否则返回 None
    pub fn get_partition_mut(&mut self, partition_id: usize) -> Option<&mut Partition> {
        self.partitions.iter_mut().find(|p| p.id == partition_id)
    }

    /// 向指定分区追加消息
    /// 
    /// # Arguments
    /// * `partition_id` - 目标分区 ID
    /// * `message` - 消息内容
    /// 
    /// # Returns
    /// * `Result<u64, String>` - 成功返回消息的偏移量，失败返回错误信息
    pub fn append_message(&self, partition_id: usize, message: Vec<u8>) -> Result<u64, String> {
        let partition = self.get_partition(partition_id)
            .ok_or_else(|| "Partition not found".to_string())?;
        
        let mut queue = partition.queue.lock()
            .map_err(|e| e.to_string())?;
            
        queue.append_message(&message)
            .map_err(|e| e.to_string())
    }

    /// 从指定分区读取消息
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// * `offset` - 消息偏移量
    /// 
    /// # Returns
    /// * `Result<Option<Vec<u8>>, String>` - 成功返回消息内容，失败返回错误信息
    pub fn read_message(&self, partition_id: usize, offset: u64) -> Result<Option<Vec<u8>>, String> {
        let partition = self.get_partition(partition_id)
            .ok_or_else(|| "Partition not found".to_string())?;
        
        let mut queue = partition.queue.lock()
            .map_err(|e| e.to_string())?;
            
        queue.read_message(offset)
            .map_err(|e| e.to_string())
    }

    /// 获取主题的分区数量
    /// 
    /// # Returns
    /// * `usize` - 分区数量
    pub fn get_partition_count(&self) -> usize {
        self.partitions.len()
    }

    /// 获取主题名称
    /// 
    /// # Returns
    /// * `&str` - 主题名称
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// 为 Topic 实现 Display trait，用于格式化输出
impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Topic: {} ({} partitions)", self.name, self.partitions.len())
    }
}
