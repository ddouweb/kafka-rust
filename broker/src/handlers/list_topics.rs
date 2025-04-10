use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 列出主题处理器
pub struct ListTopicsHandler;

impl MessageHandler for ListTopicsHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理列出主题请求: {:?}", message);
        Some(message)
    }
} 