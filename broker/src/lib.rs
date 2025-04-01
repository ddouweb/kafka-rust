pub mod metadata;
pub mod topic;
pub mod broker;
pub mod request;

// 对外暴露的核心接口
pub use broker::Broker;
pub use request::RequestHandler;
pub use metadata::{TopicConfig, PartitionMetadata, TopicMetadata, MetadataManager};
pub use topic::Topic;

// 重新导出协议类型
pub use protocol::{
    ClientRequest,
    ProduceRequest,
    FetchRequest,
    MetadataRequest,
    OffsetFetchRequest,
    JoinGroupRequest,
    SyncGroupRequest,
};