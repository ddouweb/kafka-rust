use std::env;
use cfg::ConfigStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_new_with_default_run_mode() {
        // Ensure RUN_MODE is not set
        env::remove_var("RUN_MODE");

        // Create a new Settings instance
        let settings = ConfigStruct::new();
        assert!(settings.is_ok());
        match settings {
            Ok(cfg) => {
                cfg.broker.host;
                assert_eq!(cfg.storage.segment_size, 1048576);
            },
            Err(e) => {
                eprintln!("error: {:?}", e);
            },
        }
    }

    #[test]
    fn test_load_config() {
        let config = ConfigStruct::new().expect("Failed to load config");
        
        // 打印 Broker 配置
        println!("\n=== Broker 配置 ===");
        println!("ID: {}", config.broker.id);
        println!("主机地址: {}", config.broker.host);
        println!("端口: {}", config.broker.port);
        println!("网络线程数: {}", config.broker.num_network_threads);
        println!("IO线程数: {}", config.broker.num_io_threads);
        println!("发送缓冲区大小: {} 字节", config.broker.socket_send_buffer_bytes);
        println!("接收缓冲区大小: {} 字节", config.broker.socket_receive_buffer_bytes);
        println!("最大请求大小: {} 字节", config.broker.socket_request_max_bytes);
        println!("默认分区数: {}", config.broker.num_partitions);
        println!("默认副本因子: {}", config.broker.default_replication_factor);
        println!("ZooKeeper连接: {}", config.broker.zookeeper_connect);
        println!("日志保留时间: {} 小时", config.broker.log_retention_hours);
        println!("日志段大小: {} 字节", config.broker.log_segment_bytes);
        println!("消费者组最小会话超时: {} ms", config.broker.group_min_session_timeout_ms);
        println!("消费者组最大会话超时: {} ms", config.broker.group_max_session_timeout_ms);

        // 打印存储配置
        println!("\n=== 存储配置 ===");
        println!("日志目录: {}", config.storage.log_dir);
        println!("段大小: {} 字节", config.storage.segment_size);
        println!("刷新间隔: {} ms", config.storage.flush_interval_ms);
        println!("自动创建Topic: {}", config.storage.auto_create_topics_enable);
        println!("允许删除Topic: {}", config.storage.delete_topic_enable);
        println!("压缩类型: {}", config.storage.compression_type);
        println!("最大消息大小: {} 字节", config.storage.message_max_bytes);
        println!("后台线程数: {}", config.storage.num_background_threads);
        println!("副本获取最大大小: {} 字节", config.storage.replica_fetch_max_bytes);
        println!("副本Socket超时: {} ms", config.storage.replica_socket_timeout_ms);
    }

    #[test]
    fn test_custom_config() {
        // 保存原始环境变量值
        let original_host = env::var("APP__BROKER__HOST").ok();
        let original_port = env::var("APP__BROKER__PORT").ok();
        let original_log_dir = env::var("APP__STORAGE__LOG_DIR").ok();

        // 设置环境变量来覆盖默认配置
        env::set_var("APP__BROKER__HOST", "0.0.0.0");
        env::set_var("APP__BROKER__PORT", "9093");
        env::set_var("APP__STORAGE__LOG_DIR", "./data");
        
        // 打印所有相关的环境变量值
        println!("\n=== 环境变量设置 ===");
        println!("APP__BROKER__HOST: {:?}", env::var("APP__BROKER__HOST"));
        println!("APP__BROKER__PORT: {:?}", env::var("APP__BROKER__PORT"));
        println!("APP__STORAGE__LOG_DIR: {:?}", env::var("APP__STORAGE__LOG_DIR"));
        
        let config = match ConfigStruct::new() {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("配置加载失败: {:?}", e);
                panic!("配置加载失败");
            }
        };
        
        // 打印实际值，用于调试
        println!("\n=== 配置值 ===");
        println!("期望的日志目录: ./data");
        println!("实际的日志目录: {}", config.storage.log_dir);
        println!("环境变量值: {:?}", env::var("APP__STORAGE__LOG_DIR"));
        
        // 验证环境变量覆盖是否生效
        assert_eq!(config.broker.host, "0.0.0.0", "Broker host should be 0.0.0.0");
        assert_eq!(config.broker.port, 9093, "Broker port should be 9093");
        assert_eq!(config.storage.log_dir, "./data", "Log directory should be ./data");
        
        // 验证其他配置仍使用默认值
        assert_eq!(config.broker.id, 1, "Broker ID should be 1");
        assert_eq!(config.broker.num_network_threads, 3, "Network threads should be 3");
        assert_eq!(config.storage.segment_size, 1048576, "Segment size should be 1048576");

        // 恢复原始环境变量
        if let Some(host) = original_host {
            env::set_var("APP__BROKER__HOST", host);
        } else {
            env::remove_var("APP__BROKER__HOST");
        }
        if let Some(port) = original_port {
            env::set_var("APP__BROKER__PORT", port);
        } else {
            env::remove_var("APP__BROKER__PORT");
        }
        if let Some(log_dir) = original_log_dir {
            env::set_var("APP__STORAGE__LOG_DIR", log_dir);
        } else {
            env::remove_var("APP__STORAGE__LOG_DIR");
        }
    }
}
