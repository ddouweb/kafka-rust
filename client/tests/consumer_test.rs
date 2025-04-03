use client::{Consumer, ConsumerGroup};

#[test]
fn test_consumer_creation() {
    let consumer = Consumer::new("test_consumer".to_string(), "test_group".to_string());
    
    assert_eq!(consumer.get_consumer_id(), "test_consumer");
    assert_eq!(consumer.get_group().group_id, "test_group");
    assert_eq!(consumer.get_group().members.len(), 1);
}

#[test]
fn test_consumer_group_management() {
    let mut consumer = Consumer::new("consumer1".to_string(), "group1".to_string());
    let group = consumer.get_group_mut();
    
    // 添加新成员
    group.add_member("consumer2".to_string());
    assert_eq!(group.members.len(), 2);
    
    // 移除成员
    group.remove_member("consumer2");
    assert_eq!(group.members.len(), 1);
}

#[test]
fn test_offset_management() {
    let mut consumer = Consumer::new("test_consumer".to_string(), "test_group".to_string());
    
    // 初始偏移量应该是0
    assert_eq!(consumer.get_offset(0), 0);
    
    // 更新偏移量
    consumer.update_offset(0, 100);
    assert_eq!(consumer.get_offset(0), 100);
    
    // 未设置的分区偏移量应该是0
    assert_eq!(consumer.get_offset(1), 0);
}
