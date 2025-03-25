use network::message::BinaryMessage;
pub struct Producer {
    broker_list: Vec<String>, // Kafka brokers 地址
    topic: String,            // 生产者关注的主题
    partition: Option<u32>,   // 可选的分区号
}

impl Producer {
    pub fn new(broker_list: Vec<String>, topic: String) -> Self {
        Self {
            broker_list,
            topic,
            partition: None,
        }
    }

    pub fn produce_message(&self, message: BinaryMessage) -> Result<(), String> {
        // 这里应该选择合适的 broker，并通过 `network` 库发送消息
        // 例如，使用某种分区算法选择目标分区
        let partition = self.choose_partition();
        let encoded_message = encode_message(message);

        // 使用 network 库发送消息
        send_to_broker(self.broker_list[0].clone(), partition, encoded_message)?;

        Ok(())
    }

    fn choose_partition(&self) -> u32 {
        // 实现分区选择算法
        // 这里可以使用哈希或者其他方法来决定消息的目标分区
        0
    }
}

fn encode_message(message: BinaryMessage) -> Vec<u8> {
    network::message::BinaryMessage::encode(&message)
}

fn send_to_broker(broker: String, partition: u32, message: Vec<u8>) -> Result<(), String> {
    // 通过 `network` 库与 Kafka broker 进行通信
    // 此处省略具体实现，假设 `send_request` 负责发送请求
    //send_request(broker, partition, message)
    //network::message::send_message(&mut TcpStream::connect(broker).unwrap(), &BinaryMessage::decode(&message).unwrap()).unwrap();
    todo!("未实现")
}
