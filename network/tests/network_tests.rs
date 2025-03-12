use std::vec;

use network::message::BinaryMessage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::test]
async fn test_send_message() {
    let addr = "127.0.0.1:9092";

    // 模拟客户端连接服务器
    let mut stream = TcpStream::connect(addr).await.unwrap();

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let original_msg = BinaryMessage {
        msg_type: 1,
        msg_id: 42,
        payload: vec,
    };

    stream.write_all(&original_msg.encode()).await.unwrap();

    // 读取服务器返回的数据
    let mut buffer = [0; 10240];
    let n = stream.read(&mut buffer).await.unwrap();

    let mut cursor = &buffer[..n];
    let response = match BinaryMessage::decode(&mut cursor) {
        Ok(binary_message) => {
            match String::from_utf8(binary_message.payload) {
                Ok(s) => s,
                Err(e) => format!("Invalid UTF-8 sequence: {}", e),
            }
        }
        Err(e) => format!("Error decoding message: {}", e)
    };
    //let response = String::from_utf8_lossy(&buffer[..n]);
    assert_eq!(response, "Hello, Client, message is reviced !");
}
