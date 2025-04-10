use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 创建主题处理器
pub struct CreateTopicHandler;

impl MessageHandler for CreateTopicHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理创建主题请求: {:?}", message);
        Some(message)
    }
} 