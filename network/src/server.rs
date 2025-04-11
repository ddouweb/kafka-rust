use tokio::net::TcpListener;
use tokio::io::ErrorKind;
use protocol::MessageHandler;
use std::sync::Arc;
use tokio::sync::Mutex;
use protocol::message::{MessageType, BinaryMessage};
use std::collections::HashMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::io;
use tokio::time::timeout;

pub struct NetworkServer {
    address: String,
    handlers: Arc<Mutex<HashMap<MessageType, Box<dyn MessageHandler>>>>,
    connection_timeout: Duration,
}

impl NetworkServer {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            handlers: Arc::new(Mutex::new(HashMap::new())),
            connection_timeout: Duration::from_secs(30),
        }
    }

    /// 设置连接超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// 注册消息处理器
    pub async fn register_handler(&self, message_type: MessageType, handler: Box<dyn MessageHandler>) {
        let mut handlers = self.handlers.lock().await;
        handlers.insert(message_type, handler);
    }

    /// 从流中接收消息
    pub async fn receive_message(&self, stream: &mut TcpStream) -> io::Result<BinaryMessage> {
        // 读取消息长度（4字节）
        let mut length_buf = [0u8; 4];
        timeout(self.connection_timeout, stream.read_exact(&mut length_buf))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Read timeout"))??;
        let length = u32::from_be_bytes(length_buf) as usize;

        // 读取消息内容
        let mut buffer = vec![0u8; length];
        timeout(self.connection_timeout, stream.read_exact(&mut buffer))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Read timeout"))??;

        // 解析消息
        BinaryMessage::decode_buffer(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// 发送消息到流
    pub async fn send_message(&self, stream: &mut TcpStream, message: &BinaryMessage) -> io::Result<()> {
        let encoded = message.encode();
        timeout(self.connection_timeout, stream.write_all(&encoded))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Write timeout"))??;
        timeout(self.connection_timeout, stream.flush())
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Flush timeout"))??;
        Ok(())
    }

    pub async fn start(&self) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("🚀 Server running on {}", self.address);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("📡 New connection: {}", addr);

            let handlers = Arc::clone(&self.handlers);
            let server = self.clone();
            
            tokio::spawn(async move {
                loop {
                    match server.receive_message(&mut socket).await {
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
                                if let Err(e) = server.send_message(&mut socket, &response).await {
                                    eprintln!("Error sending message: {}", e);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            if e.kind() == ErrorKind::UnexpectedEof || e.kind() == ErrorKind::ConnectionReset {
                                println!("❌ Client {} disconnected.", addr);
                            } else if e.kind() == ErrorKind::TimedOut {
                                println!("⏰ Connection timeout for client {}", addr);
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

impl Clone for NetworkServer {
    fn clone(&self) -> Self {
        Self {
            address: self.address.clone(),
            handlers: Arc::clone(&self.handlers),
            connection_timeout: self.connection_timeout,
        }
    }
}