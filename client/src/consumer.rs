use std::collections::HashMap;
use crate::group::ConsumerGroup;

/// 消费者
pub struct Consumer {
    /// 消费者ID
    consumer_id: String,
    /// 消费者组
    group: ConsumerGroup,
    /// 分区偏移量映射
    offsets: HashMap<usize, u64>,
}

impl Consumer {
    /// 创建新的消费者
    pub fn new(consumer_id: String, group_id: String) -> Self {
        let mut group = ConsumerGroup::new(group_id);
        group.add_member(consumer_id.clone());
        
        Self {
            consumer_id,
            group,
            offsets: HashMap::new(),
        }
    }

    /// 获取消费者ID
    pub fn get_consumer_id(&self) -> &str {
        &self.consumer_id
    }

    /// 获取消费者组
    pub fn get_group(&self) -> &ConsumerGroup {
        &self.group
    }

    /// 获取消费者组（可变引用）
    pub fn get_group_mut(&mut self) -> &mut ConsumerGroup {
        &mut self.group
    }

    /// 获取分区的偏移量
    pub fn get_offset(&self, partition_id: usize) -> u64 {
        *self.offsets.get(&partition_id).unwrap_or(&0)
    }

    /// 更新分区的偏移量
    pub fn update_offset(&mut self, partition_id: usize, offset: u64) {
        self.offsets.insert(partition_id, offset);
    }
}
