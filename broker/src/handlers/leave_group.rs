use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 离开消费者组处理器
pub struct LeaveGroupHandler;

impl MessageHandler for LeaveGroupHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理离开消费者组请求: {:?}", message);
        Some(message)
    }
} 