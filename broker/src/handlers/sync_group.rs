use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 同步消费者组处理器
pub struct SyncGroupHandler;

impl MessageHandler for SyncGroupHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理同步消费者组请求: {:?}", message);
        Some(message)
    }
} 