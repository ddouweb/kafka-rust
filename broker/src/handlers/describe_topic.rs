use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 描述主题处理器
pub struct DescribeTopicHandler;

impl MessageHandler for DescribeTopicHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理描述主题请求: {:?}", message);
        Some(message)
    }
} 