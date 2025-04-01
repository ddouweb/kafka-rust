use std::sync::{Arc, Mutex};
use queue::LogQueue;
use crate::metadata::{TopicConfig, PartitionMetadata};
use std::fmt;
use std::collections::HashMap;

/// 主题，代表一个消息主题，包含多个分区
#[derive(Debug)]
pub struct Topic {
    /// 主题名称
    name: String,
    /// 主题的配置信息
    config: TopicConfig,
    /// 主题的分区列表
    partitions: HashMap<usize, Arc<Mutex<LogQueue>>>,
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
            config,
            partitions: HashMap::new(),
        }
    }

    /// 获取主题的名称
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 获取主题的分区数量
    pub fn get_partition_count(&self) -> usize {
        self.partitions.len()
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
        if self.partitions.contains_key(&partition_id) {
            return Err(format!("分区 {} 已存在", partition_id));
        }

        // 创建分区目录
        let partition_dir = format!("{}/{}-{}", self.config.base_dir, self.name, partition_id);
        std::fs::create_dir_all(&partition_dir).map_err(|e| format!("创建分区目录失败: {}", e))?;

        let queue = LogQueue::new(&partition_dir, self.config.segment_size)
            .map_err(|e| format!("创建消息队列失败: {}", e))?;
        self.partitions.insert(partition_id, Arc::new(Mutex::new(queue)));
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
        if !self.partitions.contains_key(&partition_id) {
            return Err(format!("分区 {} 不存在", partition_id));
        }

        self.partitions.remove(&partition_id);
        Ok(())
    }

    /// 向指定分区追加消息
    /// 
    /// # Arguments
    /// * `partition_id` - 目标分区 ID
    /// * `message` - 消息内容
    /// 
    /// # Returns
    /// * `Result<u64, String>` - 成功返回消息的偏移量，失败返回错误信息
    pub fn append_message(&mut self, partition_id: usize, message: Vec<u8>) -> Result<u64, String> {
        let queue = self.partitions.get(&partition_id)
            .ok_or_else(|| format!("分区 {} 不存在", partition_id))?;
        
        let mut queue = queue.lock()
            .map_err(|e| format!("获取队列锁失败: {}", e))?;
            
        queue.append_message(&message)
            .map_err(|e| format!("写入消息失败: {}", e))
    }

    /// 从指定分区读取消息
    /// 
    /// # Arguments
    /// * `partition_id` - 分区 ID
    /// * `offset` - 消息偏移量
    /// 
    /// # Returns
    /// * `Result<Option<Vec<u8>>, String>` - 成功返回消息内容，失败返回错误信息
    pub fn read_message(&mut self, partition_id: usize, offset: u64) -> Result<Option<Vec<u8>>, String> {
        let queue = self.partitions.get(&partition_id)
            .ok_or_else(|| format!("分区 {} 不存在", partition_id))?;
            
        let mut queue = queue.lock()
            .map_err(|e| format!("获取队列锁失败: {}", e))?;
            
        queue.read_message(offset)
            .map_err(|e| format!("读取消息失败: {}", e))
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Topic: {} ({} partitions)", self.name, self.partitions.len())
    }
}