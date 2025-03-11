use network::NetworkServer;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_network_server() {
    let server = NetworkServer::new("127.0.0.1:9093");

    // 启动服务器
    tokio::spawn(async move {
        server.start().await.unwrap();
    });

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 模拟客户端连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:9093").await.unwrap();
    let message = "Test Message";
    stream.write_all(message.as_bytes()).await.unwrap();

    // 读取服务器返回的数据
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();
    let response = String::from_utf8_lossy(&buffer[..n]);

    assert_eq!(response, format!("ACK: {}", message));
}
