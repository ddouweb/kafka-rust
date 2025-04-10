mod produce;
mod fetch;
mod metadata;
mod offset_fetch;
mod join_group;
mod sync_group;
mod create_topic;
mod delete_topic;
mod describe_topic;
mod list_topics;
mod update_topic_config;
mod get_cluster_info;
mod heartbeat;
mod leave_group;

pub use produce::ProduceHandler;
pub use fetch::FetchHandler;
pub use metadata::MetadataHandler;
pub use offset_fetch::OffsetFetchHandler;
pub use join_group::JoinGroupHandler;
pub use sync_group::SyncGroupHandler;
pub use create_topic::CreateTopicHandler;
pub use delete_topic::DeleteTopicHandler;
pub use describe_topic::DescribeTopicHandler;
pub use list_topics::ListTopicsHandler;
pub use update_topic_config::UpdateTopicConfigHandler;
pub use get_cluster_info::GetClusterInfoHandler;
pub use heartbeat::HeartbeatHandler;
pub use leave_group::LeaveGroupHandler;

use protocol::message::MessageType;

/// 注册所有必需的消息处理器
pub async fn register_all_handlers(server: &dyn network::NetworkServer) {
    server.register_handler(MessageType::Produce, Box::new(ProduceHandler)).await;
    server.register_handler(MessageType::Fetch, Box::new(FetchHandler)).await;
    server.register_handler(MessageType::Metadata, Box::new(MetadataHandler)).await;
    server.register_handler(MessageType::OffsetFetch, Box::new(OffsetFetchHandler)).await;
    server.register_handler(MessageType::JoinGroup, Box::new(JoinGroupHandler)).await;
    server.register_handler(MessageType::SyncGroup, Box::new(SyncGroupHandler)).await;
    server.register_handler(MessageType::CreateTopic, Box::new(CreateTopicHandler)).await;
    server.register_handler(MessageType::DeleteTopic, Box::new(DeleteTopicHandler)).await;
    server.register_handler(MessageType::DescribeTopic, Box::new(DescribeTopicHandler)).await;
    server.register_handler(MessageType::ListTopics, Box::new(ListTopicsHandler)).await;
    server.register_handler(MessageType::UpdateTopicConfig, Box::new(UpdateTopicConfigHandler)).await;
    server.register_handler(MessageType::GetClusterInfo, Box::new(GetClusterInfoHandler)).await;
    server.register_handler(MessageType::Heartbeat, Box::new(HeartbeatHandler)).await;
    server.register_handler(MessageType::LeaveGroup, Box::new(LeaveGroupHandler)).await;
} 