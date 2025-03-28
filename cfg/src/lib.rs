use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Broker 配置结构体
#[derive(Debug, Deserialize)]
pub struct BrokerConfig {
    /// Broker 的唯一标识符
    pub id: u32,
    /// Broker 监听的主机地址
    pub host: String,
    /// Broker 监听的端口号
    pub port: u16,
    /// 处理网络请求的线程数
    pub num_network_threads: u32,
    /// 处理磁盘 I/O 的线程数
    pub num_io_threads: u32,
    /// 发送缓冲区大小（字节）
    pub socket_send_buffer_bytes: i32,
    /// 接收缓冲区大小（字节）
    pub socket_receive_buffer_bytes: i32,
    /// 单个请求的最大大小（字节）
    pub socket_request_max_bytes: i32,
    /// 每个 Topic 的默认分区数
    pub num_partitions: u32,
    /// 默认的副本因子（每个分区的副本数）
    pub default_replication_factor: u32,
    /// 消费者组偏移量 Topic 的副本因子
    pub offsets_topic_replication_factor: u32,
    /// 事务状态日志的副本因子
    pub transaction_state_log_replication_factor: u32,
    /// 事务状态日志的最小 ISR（同步副本）数量
    pub transaction_state_log_min_isr: u32,
    /// 日志保留时间（小时）
    pub log_retention_hours: u32,
    /// 日志保留大小（字节），-1 表示无限制
    pub log_retention_bytes: i64,
    /// 单个日志段的最大大小（字节）
    pub log_segment_bytes: i64,
    /// 检查日志保留策略的时间间隔（毫秒）
    pub log_retention_check_interval_ms: u32,
    /// ZooKeeper 连接字符串
    pub zookeeper_connect: String,
    /// ZooKeeper 连接超时时间（毫秒）
    pub zookeeper_connection_timeout_ms: u32,
    /// 消费者组初始重平衡延迟（毫秒）
    pub group_initial_rebalance_delay_ms: u32,
    /// 消费者组最小会话超时时间（毫秒）
    pub group_min_session_timeout_ms: u32,
    /// 消费者组最大会话超时时间（毫秒）
    pub group_max_session_timeout_ms: u32,
}

/// 存储配置结构体
#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    /// 日志文件存储目录
    pub log_dir: String,
    /// 单个日志段的大小（字节）
    pub segment_size: usize,
    /// 日志刷新到磁盘的时间间隔（毫秒）
    pub flush_interval_ms: u32,
    /// 日志刷新调度器的时间间隔（毫秒）
    pub flush_scheduler_interval_ms: u32,
    /// 每个数据目录的恢复线程数
    pub num_recovery_threads_per_data_dir: u32,
    /// 分区恢复线程数
    pub num_partition_recovery_threads: u32,
    /// 是否允许自动创建 Topic
    pub auto_create_topics_enable: bool,
    /// 是否允许删除 Topic
    pub delete_topic_enable: bool,
    /// 是否启用后台线程
    pub background_threads_enable: bool,
    /// 后台线程数量
    pub num_background_threads: u32,
    /// 消息压缩类型（none, gzip, snappy, lz4, zstd）
    pub compression_type: String,
    /// 单条消息的最大大小（字节）
    pub message_max_bytes: i32,
    /// 副本获取数据的最大大小（字节）
    pub replica_fetch_max_bytes: i32,
    /// 副本获取数据的最小大小（字节）
    pub replica_fetch_min_bytes: i32,
    /// 副本获取数据的最大等待时间（毫秒）
    pub replica_fetch_wait_max_ms: u32,
    /// 副本高水位标记检查点间隔（毫秒）
    pub replica_high_watermark_checkpoint_interval_ms: u32,
    /// 副本 Socket 超时时间（毫秒）
    pub replica_socket_timeout_ms: u32,
    /// 副本接收缓冲区大小（字节）
    pub replica_socket_receive_buffer_bytes: i32,
    /// 副本发送缓冲区大小（字节）
    pub replica_socket_send_buffer_bytes: i32,
    /// 副本延迟最大时间（毫秒）
    pub replica_lag_time_max_ms: u32,
    /// 副本延迟最大消息数
    pub replica_lag_max_messages: u32,
}

/// 总配置结构体
#[derive(Debug, Deserialize)]
pub struct ConfigStruct {
    /// Broker 配置
    pub broker: BrokerConfig,
    /// 存储配置
    pub storage: StorageConfig,
}

impl ConfigStruct {
    /// 创建新的配置实例
    ///
    /// 配置加载顺序：
    /// 1. 从 cfg.toml 文件加载
    /// 2. 从环境变量加载（使用 APP_ 前缀）
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("broker.id", 1)?
            .set_default("broker.host", "127.0.0.1")?
            .set_default("broker.port", 9092)?
            .set_default("broker.num_network_threads", 3)?
            .set_default("broker.num_io_threads", 8)?
            .set_default("broker.socket_send_buffer_bytes", 102400)?
            .set_default("broker.socket_receive_buffer_bytes", 102400)?
            .set_default("broker.socket_request_max_bytes", 104857600)?
            .set_default("broker.num_partitions", 3)?
            .set_default("broker.default_replication_factor", 3)?
            .set_default("broker.offsets_topic_replication_factor", 3)?
            .set_default("broker.transaction_state_log_replication_factor", 3)?
            .set_default("broker.transaction_state_log_min_isr", 2)?
            .set_default("broker.log_retention_hours", 168)?
            .set_default("broker.log_retention_bytes", -1)?
            .set_default("broker.log_segment_bytes", 1073741824)?
            .set_default("broker.log_retention_check_interval_ms", 300000)?
            .set_default("broker.zookeeper_connect", "localhost:2181")?
            .set_default("broker.zookeeper_connection_timeout_ms", 18000)?
            .set_default("broker.group_initial_rebalance_delay_ms", 0)?
            .set_default("broker.group_min_session_timeout_ms", 6000)?
            .set_default("broker.group_max_session_timeout_ms", 300000)?
            // 存储配置默认值
            .set_default("storage.log_dir", "/var/lib/rust_kafka")?
            .set_default("storage.segment_size", 1048576)?
            .set_default("storage.flush_interval_ms", 1000)?
            .set_default("storage.flush_scheduler_interval_ms", 3000)?
            .set_default("storage.num_recovery_threads_per_data_dir", 1)?
            .set_default("storage.num_partition_recovery_threads", 1)?
            .set_default("storage.auto_create_topics_enable", true)?
            .set_default("storage.delete_topic_enable", true)?
            .set_default("storage.background_threads_enable", true)?
            .set_default("storage.num_background_threads", 10)?
            .set_default("storage.compression_type", "producer")?
            .set_default("storage.message_max_bytes", 1000012)?
            .set_default("storage.replica_fetch_max_bytes", 1048576)?
            .set_default("storage.replica_fetch_min_bytes", 1)?
            .set_default("storage.replica_fetch_wait_max_ms", 500)?
            .set_default(
                "storage.replica_high_watermark_checkpoint_interval_ms",
                5000,
            )?
            .set_default("storage.replica_socket_timeout_ms", 30000)?
            .set_default("storage.replica_socket_receive_buffer_bytes", 65536)?
            .set_default("storage.replica_socket_send_buffer_bytes", 65536)?
            .set_default("storage.replica_lag_time_max_ms", 10000)?
            .set_default("storage.replica_lag_max_messages", 4000)?
            .add_source(File::with_name("cfg").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        println!("Debug: 完整配置: {:?}", config);
        
        config.try_deserialize::<ConfigStruct>()
    }

}
