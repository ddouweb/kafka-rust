use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 消费消息处理器
pub struct FetchHandler;

impl MessageHandler for FetchHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理消费消息请求: {:?}", message);
        Some(message)
    }
} 