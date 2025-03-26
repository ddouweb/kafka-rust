pub mod message;
mod server;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
pub use message::BinaryMessage;
pub use server::NetworkServer;


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