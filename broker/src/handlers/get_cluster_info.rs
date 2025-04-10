use protocol::message::BinaryMessage;
use protocol::MessageHandler;

/// 获取集群信息处理器
pub struct GetClusterInfoHandler;

impl MessageHandler for GetClusterInfoHandler {
    fn handle_message(&self, message: BinaryMessage) -> Option<BinaryMessage> {
        println!("处理获取集群信息请求: {:?}", message);
        Some(message)
    }
} 