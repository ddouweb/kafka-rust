use std::sync::{Arc, Mutex};
use queue::LogQueue;
use crate::metadata::{TopicConfig, PartitionMetadata};
use std::fmt;
use std::collections::HashMap;
use std::time::Instant;

/// 分区状态
#[derive(Debug)]
enum PartitionState {
    Active,
    Deleted(Instant), // 记录删除时间
}

/// 主题，代表一个消息主题，包含多个分区
#[derive(Debug)]
pub struct Topic {
    /// 主题名称
    name: String,
    /// 主题的配置信息
    config: TopicConfig,
    /// 主题的分区列表
    partitions: HashMap<usize, (Arc<Mutex<LogQueue>>, PartitionState)>,
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
    pub fn create_partition(&mut self, partition_id: usize, _metadata: PartitionMetadata) -> Result<(), String> {
        if self.partitions.contains_key(&partition_id) {
            return Err(format!("分区 {} 已存在", partition_id));
        }

        //如果分区号大于等于分区数量，则返回错误
        if partition_id >= self.config.partitions {
            return Err(format!("分区号 {} 错误", partition_id));
        }

        // 创建分区目录
        let partition_dir = self.get_partition_dir(partition_id);

        if !std::path::Path::new(&partition_dir).exists() {
            std::fs::create_dir_all(&partition_dir).map_err(|e| format!("创建分区目录失败: {}", e))?;
        }

        let queue = LogQueue::new(&partition_dir, self.config.segment_size)
            .map_err(|e| format!("创建消息队列失败: {}", e))?;
        self.partitions.insert(partition_id, (Arc::new(Mutex::new(queue)), PartitionState::Active));
        Ok(())
    }

    //初始化分区
    pub fn init_partitions(&mut self) -> Result<(), String> {
        for i in 0..self.config.partitions {
            self.create_partition(i, PartitionMetadata {
                id: i,
                leader: 0,
                replicas: vec![0],
                isr: vec![0],
            })?;
        }
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

        // 标记分区为已删除状态
        if let Some((_queue, state)) = self.partitions.get_mut(&partition_id) {
            match state {
                PartitionState::Active => {
                    *state = PartitionState::Deleted(Instant::now());
                    self.partitions.remove(&partition_id); //移除分区信息
                    //let _ = self.cleanup_deleted_partitions(100);
                    Ok(())
                }
                PartitionState::Deleted(_) => Err(format!("分区 {} 已经被标记为删除", partition_id)),
            }
        } else {
            Err(format!("分区 {} 不存在", partition_id))
        }
    }

    /// 删除整个主题
    /// 
    /// # Returns
    /// * `Result<(), String>` - 删除成功返回 Ok(()), 失败返回错误信息
    pub fn delete_topic(&mut self) -> Result<(), String> {
        let partition_ids: Vec<usize> = self.partitions.keys().copied().collect();
        for partition_id in partition_ids {
            self.delete_partition(partition_id)?;
            let partition_dir = self.get_partition_dir(partition_id);
            if let Err(e) = std::fs::remove_dir_all(&partition_dir) {
                return Err(format!("删除分区目录失败: {}", e));
            }
        }
        Ok(())
    }

    /// 删除所有分区
    pub fn delete_all_partitions(&mut self) -> Result<(), String> {
        let partition_ids: Vec<usize> = self.partitions.keys().copied().collect();
        for partition_id in partition_ids {
            self.delete_partition(partition_id)?;
        }
        Ok(())
    }

    /// 清理已标记为删除的分区
    /// 
    /// # Arguments
    /// * `max_age_seconds` - 分区被标记为删除后的最大保留时间（秒）
    pub fn cleanup_deleted_partitions(&mut self, max_age_seconds: u64) -> Result<(), String> {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        for (partition_id, (_queue, state)) in self.partitions.iter() {
            if let PartitionState::Deleted(deleted_time) = state {
                if now.duration_since(*deleted_time).as_secs() >= max_age_seconds {
                    to_remove.push(*partition_id);
                }
            }
        }

        for partition_id in to_remove {
            // 删除物理目录
            let partition_dir = self.get_partition_dir(partition_id);
            if let Err(e) = std::fs::remove_dir_all(&partition_dir) {
                eprintln!("删除分区目录失败: {}", e);
                continue;
            }
            self.partitions.remove(&partition_id);
        }

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
        let (queue, state) = self.partitions.get(&partition_id)
            .ok_or_else(|| format!("分区 {} 不存在", partition_id))?;
            
        match state {
            PartitionState::Active => {
                let mut queue = queue.lock()
                    .map_err(|e| format!("获取队列锁失败: {}", e))?;
                    
                queue.append_message(&message)
                    .map_err(|e| format!("写入消息失败: {}", e))
            }
            PartitionState::Deleted(_) => Err(format!("分区 {} 已被标记为删除", partition_id)),
        }
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
        let (queue, state) = self.partitions.get(&partition_id)
            .ok_or_else(|| format!("分区 {} 不存在", partition_id))?;
            
        match state {
            PartitionState::Active => {
                let mut queue = queue.lock()
                    .map_err(|e| format!("获取队列锁失败: {}", e))?;
                    
                queue.read_message(offset)
                    .map_err(|e| format!("读取消息失败: {}", e))
            }
            PartitionState::Deleted(_) => Err(format!("分区 {} 已被标记为删除", partition_id)),
        }
    }

    //返回分区目录
    pub fn get_partition_dir(&self, partition_id: usize) -> String {
        format!("{}/{}-{}", self.config.base_dir, self.name, partition_id)
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Topic: {} ({} partitions)", self.name, self.partitions.len())
    }
}