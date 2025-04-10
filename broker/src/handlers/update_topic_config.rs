use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 更新主题配置处理器
pub struct UpdateTopicConfigHandler;

impl MessageHandler for UpdateTopicConfigHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理更新主题配置请求: {:?}", message);
        Some(message)
    }
} 