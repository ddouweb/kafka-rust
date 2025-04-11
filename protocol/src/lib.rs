//! Kafka 协议实现
//! 
//! 该模块提供了 Kafka 协议的 Rust 实现，包括：
//! - 消息类型定义
//! - 请求/响应处理
//! - 二进制消息编解码

pub mod message;
pub mod request;
pub mod response;

// 导出常用类型
pub use message::{MessageType, BinaryMessage};
pub use request::{ClientRequest, ProduceRequest, FetchRequest};
pub use response::ServerResponse;

// 导出错误类型
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ProtocolError {
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        #[error("Serialization error: {0}")]
        Serialization(#[from] serde_json::Error),
        #[error("Invalid message type: {0}")]
        InvalidMessageType(u8),
        #[error("Invalid message format")]
        InvalidMessageFormat,
        #[error("Buffer too short")]
        BufferTooShort,
    }
}

pub type Result<T> = std::result::Result<T, error::ProtocolError>;

/// 消息处理器trait
pub trait MessageHandler: Send + Sync {
    fn handle_message(&self, message: BinaryMessage) -> Option<message::BinaryMessage>;
}