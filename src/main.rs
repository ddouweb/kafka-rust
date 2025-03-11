use network::NetworkServer;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let server = NetworkServer::new("127.0.0.1:9092");

    rt.block_on(server.start()).unwrap();
}
