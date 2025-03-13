pub mod message;

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tokio::net::TcpStream;

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
        //tokio::net::UdpSocket::bind("127.0.0.1:9092").await?;
        let listener = TcpListener::bind(&self.address).await?;
        println!("🚀 Server running on {}", self.address);

        let shared_state = Arc::new(Mutex::new(())); // 这里可以存储消息

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("📡 New connection: {}", addr);

            let _ = Arc::clone(&shared_state);
            tokio::spawn(async move {

                match receive_message(&mut socket).await {
                    Ok(binary_message) => {
                        match String::from_utf8(binary_message.payload) {
                            Ok(text) => println!("📩 Received message: {}", text),
                            Err(_) => println!("⚠️ Received non-UTF8 binary data"),
                        }

                        //println!("📩 Received message: {}", binary_message.msg_id);

                        // 创建 BinaryMessage 实例并调用 send_message
                        let response_message = BinaryMessage {
                            msg_type: 1,     // 根据需要设置类型
                            msg_id: 1234,    // 设置唯一的消息 ID
                            payload:  binary_message.payload, // 负载内容可以是空的，或者根据需要填充
                        };

                        if let Err(e) = send_message(&mut socket, &response_message).await {
                            eprintln!("Error sending message: {}", e);
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to receive message: {}", e),
                }
                
                // let mut buffer = vec![0; 1048576]; // 扩大 buffer
                // match socket.read(&mut buffer).await {
                //     Ok(n) if n > 0 => {
                //         // 示例：通过 BinaryMessage 解析二进制数据
                //         let mut cursor = &buffer[..n];
                //         match BinaryMessage::decode(&mut cursor) {
                //             Ok(binary_message) => {
                //                 println!("Decoded message: {:?}", binary_message);
                //             }
                //             Err(e) => eprintln!("Error decoding message: {}", e),
                //         }

                //         // 创建 BinaryMessage 实例并调用 send_message
                //         let response_message = BinaryMessage {
                //             msg_type: 1,     // 根据需要设置类型
                //             msg_id: 1234,    // 设置唯一的消息 ID
                //             payload:  b"Hello, Client, message is reviced !".to_vec(), // 负载内容可以是空的，或者根据需要填充
                //         };

                //         if let Err(e) = send_message(&mut socket, &response_message).await {
                //             eprintln!("Error sending message: {}", e);
                //         }
                //     }
                //     _ => println!("⚠️ Connection lost: {}", addr),
                // }
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

    let mut buffer = vec![0u8; length]; // 分配缓冲区存放消息
    stream.read_exact(&mut buffer).await?;

    let mut cursor = std::io::Cursor::new(buffer);
    BinaryMessage::decode(&mut cursor)
    .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e))
}