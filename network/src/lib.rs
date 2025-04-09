use protocol::message::BinaryMessage;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub mod server;
pub use server::NetworkServer;

/// 从流中接收消息
pub async fn receive_message(stream: &mut TcpStream) -> io::Result<BinaryMessage> {
    // 读取消息长度（4字节）
    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf).await?;
    let length = u32::from_be_bytes(length_buf) as usize;

    // 读取消息内容
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer).await?;

    // 解析消息
    BinaryMessage::decode(&buffer)
}

/// 发送消息到流
pub async fn send_message(stream: &mut TcpStream, message: &BinaryMessage) -> io::Result<()> {
    let encoded = message.encode();
    stream.write_all(&encoded).await?;
    stream.flush().await?;
    Ok(())
}