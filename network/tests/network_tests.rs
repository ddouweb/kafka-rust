use std::vec;
use protocol::{BinaryMessage, MessageType};
use tokio::net::TcpStream;
use network::send_message;
use network::receive_message;

#[tokio::test]
async fn test_send_message() {
    let addr = "127.0.0.1:9092";

    // 模拟客户端连接服务器
    let mut stream = TcpStream::connect(addr).await.unwrap();

    let original_msg = BinaryMessage {
        msg_type: MessageType::Produce,
        msg_id: 10001001,
        correlation_id: 10001002,
        client_id: 10001003,
        payload: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],

    };

    send_message(&mut stream, &original_msg)
        .await
        .expect("Failed to send message");

    let response = receive_message(&mut stream)
        .await
        .expect("Failed to read message");
    println!("{:?}", response);
    assert_eq!(response.payload, original_msg.payload);
    assert_eq!(response.msg_type, original_msg.msg_type);
    assert_eq!(response.msg_id, original_msg.msg_id);

    std::thread::sleep(std::time::Duration::new(2, 0)); // 睡眠2秒
    // stream
    //     .shutdown()
    //     .await
    //     .expect("Failed to shutdown connection");
    // drop(stream); // 完全关闭连接

    
}
