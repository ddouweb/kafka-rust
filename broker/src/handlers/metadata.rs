use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 元数据请求处理器
pub struct MetadataHandler;

impl MessageHandler for MetadataHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理元数据请求: {:?}", message);
        Some(message)
    }
} 