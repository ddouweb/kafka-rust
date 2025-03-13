pub mod message;

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tokio::net::TcpStream;
use tokio::io::ErrorKind;

use crate::message::BinaryMessage;

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
                    match receive_message(&mut socket).await {
                        Ok(binary_message) => {
                            println!("收到消息：{}",binary_message.msg_id);
                            if let Err(e) = send_message(&mut socket, &binary_message).await {
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

pub async fn send_message(stream: &mut TcpStream, msg: &BinaryMessage) -> tokio::io::Result<()> {
    let encoded = msg.encode();
    stream.write_all(&encoded).await?; // 使用异步写入
    stream.flush().await?; // 确保数据写入
    Ok(())
}

pub async fn receive_message(stream: &mut TcpStream) -> tokio::io::Result<BinaryMessage> {
    let mut length_buf = [0u8; 4]; // 读取 4 字节，表示消息总长度
    stream.read_exact(&mut length_buf).await?;
    let length = u32::from_be_bytes(length_buf) as usize;
    let mut buffer = vec![0u8; length];

    stream.read_exact(&mut buffer).await?;
    BinaryMessage::decode(&buffer)
        .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e))
}