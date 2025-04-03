use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::io::ErrorKind;

pub struct NetworkServer {
    address: String,
}

impl NetworkServer {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub async fn start(&self) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("🚀 Server running on {}", self.address);

        let shared_state = Arc::new(Mutex::new(())); // 这里可以存储消息

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("📡 New connection: {}", addr);

            let _ = Arc::clone(&shared_state);
            tokio::spawn(async move {
                loop {
                    match crate::receive_message(&mut socket).await {
                        Ok(binary_message) => {
                            println!("收到消息：{}",binary_message.msg_id);
                            if let Err(e) = crate::send_message(&mut socket, &binary_message).await {
                                eprintln!("Error sending message: {}", e);
                                break;
                            }
                        }
                        //Err(e) => eprintln!("❌ Failed to receive message: {}", e),
                        Err(e) => {
                            // ✅ 客户端断开连接
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