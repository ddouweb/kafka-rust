use network::NetworkServer;

#[tokio::main]
async fn main() {
    //let rt = Runtime::new().unwrap();
    let server = NetworkServer::new("127.0.0.1:9092");
    //handlers::register_all_handlers(&server).await;
    //rt.block_on(server.start()).unwrap();
    server.start().await.unwrap();
}