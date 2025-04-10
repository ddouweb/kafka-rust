use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 删除主题处理器
pub struct DeleteTopicHandler;

impl MessageHandler for DeleteTopicHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理删除主题请求: {:?}", message);
        Some(message)
    }
} 