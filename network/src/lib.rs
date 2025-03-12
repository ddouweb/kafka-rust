use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use std::net::TcpStream;
use crate::message::BinaryMessage;

pub mod message;

#[derive(Debug, Deserialize)]
struct Request {
    r#type: String,  // `type` 是关键字，需要用 `r#` 逃避
    topic: String,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    status: String,
    message: String,
}

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

            let state = Arc::clone(&shared_state);
            tokio::spawn(async move {
                let mut buffer = vec![0; 1048576]; // 扩大 buffer
                match socket.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        let request_str = String::from_utf8_lossy(&buffer[..n]);
                        println!("📩 Received: {}", request_str);

                        let response: Response = match serde_json::from_str::<Request>(&request_str) {
                            Ok(req) => {
                                if req.r#type == "produce" {
                                    println!("📝 Storing message in topic `{}`: {:?}", req.topic, req.message);
                                    Response { status: "ok".to_string(), message: "Message received".to_string() }
                                } else if req.r#type == "fetch" {
                                    println!("📤 Fetching messages for topic `{}`", req.topic);
                                    Response { status: "ok".to_string(), message: format!("Messages from {}", req.topic) }
                                } else {
                                    Response { status: "error".to_string(), message: "Invalid request type".to_string() }
                                }
                            }
                            Err(_) => Response { status: "error".to_string(), message: "Invalid JSON".to_string() },
                        };

                        let response_str = serde_json::to_string(&response).unwrap();
                        socket.write_all(response_str.as_bytes()).await.unwrap();
                    }
                    _ => println!("⚠️ Connection lost: {}", addr),
                }
            });
        }
    }
}


pub fn send_message(stream: &mut TcpStream, msg: &BinaryMessage) -> std::io::Result<()> {
    let encoded = msg.encode();
    stream.write_all(&encoded)?;
    Ok(())
}
pub fn receive_message(stream: &mut TcpStream) -> std::io::Result<BinaryMessage> {
    BinaryMessage::decode(stream)
}