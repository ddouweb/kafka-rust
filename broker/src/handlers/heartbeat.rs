use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 心跳处理器
pub struct HeartbeatHandler;

impl MessageHandler for HeartbeatHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理心跳请求: {:?}", message);
        Some(message)
    }
} 