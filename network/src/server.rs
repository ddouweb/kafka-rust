use tokio::net::TcpListener;
use tokio::io::ErrorKind;
use protocol::MessageHandler;
use std::sync::Arc;
use tokio::sync::Mutex;
use protocol::message::MessageType;
use std::collections::HashMap;

pub struct NetworkServer {
    address: String,
    handlers: Arc<Mutex<HashMap<MessageType, Box<dyn MessageHandler>>>>,
}

impl NetworkServer {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册消息处理器
    pub async fn register_handler(&self, message_type: MessageType, handler: Box<dyn MessageHandler>) {
        let mut handlers = self.handlers.lock().await;
        handlers.insert(message_type, handler);
    }

    pub async fn start(&self) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("🚀 Server running on {}", self.address);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("📡 New connection: {}", addr);

            let handlers = Arc::clone(&self.handlers);
            tokio::spawn(async move {
                loop {
                    match crate::receive_message(&mut socket).await {
                        Ok(binary_message) => {
                            println!("收到消息：{}", binary_message.msg_id);
                            
                            // 直接使用handlers处理消息
                            let response = {
                                let handlers = handlers.lock().await;
                                if let Some(handler) = handlers.get(&binary_message.msg_type) {
                                    handler.handle_message(binary_message)
                                } else {
                                    println!("No handler found for message type: {:?}", binary_message.msg_type);
                                    None
                                }
                            };
                            
                            if let Some(response) = response {
                                if let Err(e) = crate::send_message(&mut socket, &response).await {
                                    eprintln!("Error sending message: {}", e);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            if e.kind() == ErrorKind::UnexpectedEof || e.kind() == ErrorKind::ConnectionReset {
                                println!("❌ Client {} disconnected.", addr);
                            } else {
                                eprintln!("❌ Failed to receive message: {}", e);
                            }
                            break;
                        }
                    }
                }
            });
        }
    }

}