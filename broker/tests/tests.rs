#[cfg(test)]
mod tests {
    use broker::metadata::{TopicConfig, PartitionMetadata, TopicMetadata};
    use broker::topic::Topic;
    use broker::metadata::MetadataManager;

    #[test]
    fn test_topic_creation() {
        let config = TopicConfig {
            name: "test-topic".to_string(),
            partitions: 3,
            replication_factor: 1,
            segment_size: 1024 * 1024,
        };
        let topic = Topic::new("test-topic".to_string(), config.clone());
        assert_eq!(topic.get_name(), "test-topic");
        assert_eq!(topic.get_partition_count(), 0);
    }

    #[test]
    fn test_partition_creation() {
        let mut topic = Topic::new("test-topic".to_string(), TopicConfig {
            name: "test-topic".to_string(),
            partitions: 3,
            replication_factor: 1,
            segment_size: 1024 * 1024,
        });

        let metadata = PartitionMetadata {
            id: 0,
            leader: 0,
            replicas: vec![0],
            isr: vec![0],
        };

        assert!(topic.create_partition(0, metadata.clone()).is_ok());
        assert_eq!(topic.get_partition_count(), 1);
        
        // 测试重复创建分区
        assert!(topic.create_partition(0, metadata).is_err());
    }

    #[test]
    fn test_partition_deletion() {
        let mut topic = Topic::new("test-topic".to_string(), TopicConfig {
            name: "test-topic".to_string(),
            partitions: 3,
            replication_factor: 1,
            segment_size: 1024 * 1024,
        });

        let metadata = PartitionMetadata {
            id: 0,
            leader: 0,
            replicas: vec![0],
            isr: vec![0],
        };

        topic.create_partition(0, metadata).unwrap();
        assert!(topic.delete_partition(0).is_ok());
        assert_eq!(topic.get_partition_count(), 0);
        
        // 测试删除不存在的分区
        assert!(topic.delete_partition(0).is_err());
    }

    #[test]
    fn test_message_operations() {
        let mut topic = Topic::new("test-topic".to_string(), TopicConfig {
            name: "test-topic".to_string(),
            partitions: 3,
            replication_factor: 1,
            segment_size: 1024 * 1024,
        });

        let metadata = PartitionMetadata {
            id: 0,
            leader: 0,
            replicas: vec![0],
            isr: vec![0],
        };

        topic.create_partition(0, metadata).unwrap();

        // 测试消息写入
        let message = vec![1, 2, 3, 4, 5];
        let result = topic.append_message(0, message.clone());
        assert!(result.is_ok());

        // 测试消息读取
        let read_result = topic.read_message(0, 0);
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap().unwrap(), message);

        // 测试读取不存在的分区
        assert!(topic.read_message(1, 0).is_err());
    }

    #[test]
    fn test_metadata_manager() {
        let manager = MetadataManager::new();
        
        let config = TopicConfig {
            name: "test-topic".to_string(),
            partitions: 3,
            replication_factor: 1,
            segment_size: 1024 * 1024,
        };
        
        let mut topic_metadata = TopicMetadata::new("test-topic".to_string(), config);
        
        // 测试添加主题
        assert!(manager.add_topic(topic_metadata.clone()).is_ok());
        
        // 测试重复添加主题
        assert!(manager.add_topic(topic_metadata.clone()).is_err());
        
        // 测试获取主题
        let result = manager.get_topic("test-topic").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "test-topic");
        
        // 测试删除主题
        assert!(manager.remove_topic("test-topic").is_ok());
        
        // 验证主题已被删除
        let result = manager.get_topic("test-topic").unwrap();
        assert!(result.is_none());
    }
} 