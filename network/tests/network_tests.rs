use network::NetworkServer;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use network::BinaryMessage;

#[tokio::test]
async fn test_network_server() {
    let addr = "127.0.0.1:9093";
    let server = NetworkServer::new(addr);

    // 启动服务器
    tokio::spawn(async move {
        server.start().await.unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 模拟客户端连接服务器
    let mut stream = TcpStream::connect(addr).await.unwrap();
    let message = "Test Message";

    let original_msg = BinaryMessage {
        msg_type: 1,
        msg_id: 42,
        payload: b"Hello, Kafka!".to_vec(),
    };
    
    stream.write_all(&original_msg.encode()).await.unwrap();    

    // 读取服务器返回的数据
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();
    let response = String::from_utf8_lossy(&buffer[..n]);

    assert_eq!(response, format!("ACK: {}", message));
}
