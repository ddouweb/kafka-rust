mod broker;
mod request;
mod metadata;
mod topic;

// 对外暴露的核心接口
pub use broker::Broker;
pub use request::RequestHandler;
pub use metadata::TopicMetadata;
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