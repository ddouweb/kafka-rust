use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 偏移量获取处理器
pub struct OffsetFetchHandler;

impl MessageHandler for OffsetFetchHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理偏移量获取请求: {:?}", message);
        Some(message)
    }
} 