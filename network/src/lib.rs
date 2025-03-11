use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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

        let shared_state = Arc::new(Mutex::new(())); // 这里可以用来共享状态（如存储队列）

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("📡 New connection: {}", addr);

            let state = Arc::clone(&shared_state);
            tokio::spawn(async move {
                let mut buffer = [0; 1024];

                match socket.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        let request = String::from_utf8_lossy(&buffer[..n]);
                        println!("📩 Received: {}", request);

                        let response = format!("ACK: {}", request);
                        socket.write_all(response.as_bytes()).await.unwrap();
                    }
                    _ => println!("⚠️ Connection lost: {}", addr),
                }
            });
        }
    }
}