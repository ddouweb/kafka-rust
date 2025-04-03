use std::collections::HashMap;

/// 消费者组成员信息
#[derive(Debug, Clone)]
pub struct GroupMember {
    /// 消费者ID
    pub member_id: String,
    /// 分配的分区列表
    pub assigned_partitions: Vec<usize>,
}

/// 消费者组
#[derive(Debug)]
pub struct ConsumerGroup {
    /// 消费者组ID
    pub group_id: String,
    /// 组成员信息
    pub members: HashMap<String, GroupMember>,
    /// 分区分配信息
    pub partition_assignment: HashMap<usize, String>,
}

impl ConsumerGroup {
    /// 创建新的消费者组
    pub fn new(group_id: String) -> Self {
        Self {
            group_id,
            members: HashMap::new(),
            partition_assignment: HashMap::new(),
        }
    }

    /// 添加消费者到组
    pub fn add_member(&mut self, member_id: String) {
        self.members.insert(member_id.clone(), GroupMember {
            member_id,
            assigned_partitions: Vec::new(),
        });
    }

    /// 从组中移除消费者
    pub fn remove_member(&mut self, member_id: &str) {
        self.members.remove(member_id);
        // 清理该消费者的分区分配
        self.partition_assignment.retain(|_, v| v != member_id);
    }

    /// 获取消费者分配的分区
    pub fn get_assigned_partitions(&self, member_id: &str) -> Option<&Vec<usize>> {
        self.members.get(member_id).map(|m| &m.assigned_partitions)
    }
} 