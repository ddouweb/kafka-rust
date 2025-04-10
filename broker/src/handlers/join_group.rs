use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 加入消费者组处理器
pub struct JoinGroupHandler;

impl MessageHandler for JoinGroupHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理加入消费者组请求: {:?}", message);
        Some(message)
    }
} 