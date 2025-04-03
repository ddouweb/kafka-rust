use client::{Producer, ProducerConfig};

#[test]
fn test_producer_creation() {
    let config = ProducerConfig::default();
    let producer = Producer::new("test_producer".to_string(), config);
    
    assert_eq!(producer.get_producer_id(), "test_producer");
    assert!(producer.get_config().auto_select_partition);
    assert_eq!(producer.get_config().partition_count, 1);
}

#[test]
fn test_partition_selection() {
    let mut config = ProducerConfig::default();
    config.partition_count = 3;
    let mut producer = Producer::new("test_producer".to_string(), config);
    
    // 测试无key时的轮询分配
    let partition1 = producer.select_partition(None);
    let partition2 = producer.select_partition(None);
    let partition3 = producer.select_partition(None);
    let partition4 = producer.select_partition(None);
    
    assert_eq!(partition1, 0);
    assert_eq!(partition2, 1);
    assert_eq!(partition3, 2);
    assert_eq!(partition4, 0); // 回到第一个分区
    
    // 测试有key时的哈希分配
    let key1 = b"key1";
    let key2 = b"key2";
    let partition_key1 = producer.select_partition(Some(key1));
    let partition_key2 = producer.select_partition(Some(key2));
    
    // 相同的key应该分配到相同的分区
    assert_eq!(partition_key1, producer.select_partition(Some(key1)));
    assert_eq!(partition_key2, producer.select_partition(Some(key2)));
}

#[test]
fn test_message_sending() {
    let config = ProducerConfig::default();
    let mut producer = Producer::new("test_producer".to_string(), config);
    
    // 测试发送消息
    let message = vec![1, 2, 3];
    let result = producer.send_message(message.clone(), None);
    assert!(result.is_ok());
    
    let (partition_id, offset) = result.unwrap();
    assert_eq!(partition_id, 0); // 默认配置下只有一个分区
    assert_eq!(offset, 0); // 临时返回值
}
