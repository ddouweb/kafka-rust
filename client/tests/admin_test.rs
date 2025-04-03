use client::{AdminClient, TopicConfig};
use std::collections::HashMap;

#[test]
fn test_admin_client_creation() {
    let admin = AdminClient::new("test_admin".to_string());
    // 目前只是创建实例，没有其他可测试的属性
}

#[test]
fn test_topic_operations() {
    let admin = AdminClient::new("test_admin".to_string());
    
    // 测试创建主题
    let mut config = TopicConfig::default();
    config.name = "test_topic".to_string();
    config.num_partitions = 3;
    config.replication_factor = 1;
    
    assert!(admin.create_topic(config.clone()).is_ok());
    
    // 测试获取主题描述
    let description = admin.describe_topic(&config.name);
    assert!(description.is_ok());
    
    // 测试列出主题
    let topics = admin.list_topics();
    assert!(topics.is_ok());
    
    // 测试更新主题配置
    let mut new_configs = HashMap::new();
    new_configs.insert("retention.ms".to_string(), "604800000".to_string());
    assert!(admin.update_topic_config(&config.name, new_configs).is_ok());
    
    // 测试删除主题
    assert!(admin.delete_topic(&config.name).is_ok());
}

#[test]
fn test_cluster_operations() {
    let admin = AdminClient::new("test_admin".to_string());
    
    // 测试获取集群信息
    let cluster_info = admin.get_cluster_info();
    assert!(cluster_info.is_ok());
} 