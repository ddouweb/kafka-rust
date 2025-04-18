use network::NetworkServer;
use protocol::{BinaryMessage, MessageType};
use std::vec;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::time::Duration;

async fn setup_test_server() -> (TcpStream, TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { listener.accept().await.unwrap().0 });
    let client = TcpStream::connect(addr).await.unwrap();
    let server = server.await.unwrap();
    (server, client)
}

async fn setup_test_server_1() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let network_server = NetworkServer::new("127.0.0.1:0").with_timeout(Duration::from_secs(5));
    let client = TcpStream::connect(addr).await.unwrap();
    network_server.send_message(&mut client, &test_message).await.unwrap();

}

#[tokio::test]
async fn test_message_roundtrip() {
    let (mut server, mut client) = setup_test_server().await;
    let network_server = NetworkServer::new("127.0.0.1:0").with_timeout(Duration::from_secs(5));

    // 创建测试消息
    let test_message = BinaryMessage {
        msg_id: 1,
        msg_type: MessageType::Produce,
        payload: vec![1, 2, 3, 4],
        client_id: 1,
        correlation_id: 1,
    };

    // 发送消息
    network_server.send_message(&mut client, &test_message).await.unwrap();

    // 接收消息
    let received = network_server.receive_message(&mut server).await.unwrap();

    // 验证消息内容
    assert_eq!(received.msg_id, test_message.msg_id);
    assert_eq!(received.msg_type, test_message.msg_type);
    assert_eq!(received.payload, test_message.payload);
    assert_eq!(received.client_id, test_message.client_id);
    assert_eq!(received.correlation_id, test_message.correlation_id);
}

#[tokio::test]
async fn test_invalid_message() {
    let (mut server, mut client) = setup_test_server().await;
    let network_server = NetworkServer::new("127.0.0.1:0").with_timeout(Duration::from_secs(5));

    // 发送无效数据
    client.write_all(&[0, 0, 0, 4, 1, 2, 3]).await.unwrap();
    client.flush().await.unwrap();

    // 应该返回错误
    let result = network_server.receive_message(&mut server).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

#[tokio::test]
async fn test_concurrent_messages() {
    let (mut server, mut client) = setup_test_server().await;
    let network_server = NetworkServer::new("127.0.0.1:0").with_timeout(Duration::from_secs(5));

    // 创建两个测试消息
    let message1 = BinaryMessage {
        msg_id: 1,
        msg_type: MessageType::Produce,
        payload: vec![1, 2, 3],
        client_id: 1,
        correlation_id: 1,
    };

    let message2 = BinaryMessage {
        msg_id: 2,
        msg_type: MessageType::Fetch,
        payload: vec![4, 5, 6],
        client_id: 2,
        correlation_id: 2,
    };

    // 发送两个消息
    network_server.send_message(&mut client, &message1).await.unwrap();
    network_server.send_message(&mut client, &message2).await.unwrap();

    // 接收并验证第一个消息
    let received1 = network_server.receive_message(&mut server).await.unwrap();
    assert_eq!(received1.msg_id, message1.msg_id);
    assert_eq!(received1.msg_type, message1.msg_type);
    assert_eq!(received1.payload, message1.payload);
    assert_eq!(received1.client_id, message1.client_id);
    assert_eq!(received1.correlation_id, message1.correlation_id);

    // 接收并验证第二个消息
    let received2 = network_server.receive_message(&mut server).await.unwrap();
    assert_eq!(received2.msg_id, message2.msg_id);
    assert_eq!(received2.msg_type, message2.msg_type);
    assert_eq!(received2.payload, message2.payload);
    assert_eq!(received2.client_id, message2.client_id);
    assert_eq!(received2.correlation_id, message2.correlation_id);
}

#[tokio::test]
async fn test_timeout() {
    let (mut server, mut client) = setup_test_server().await;
    let network_server = NetworkServer::new("127.0.0.1:0").with_timeout(Duration::from_millis(100));

    // 发送部分数据
    client.write_all(&[0, 0, 0, 4]).await.unwrap();
    client.flush().await.unwrap();

    // 应该超时
    let result = network_server.receive_message(&mut server).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::TimedOut);
}
