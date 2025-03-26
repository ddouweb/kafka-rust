use network::BinaryMessage;
use crate::broker::Broker;

pub struct RequestHandler {
    broker: Broker,
}

impl RequestHandler {
    pub fn new(broker: Broker) -> Self {
        Self { broker }
    }

    // pub fn handle_request(&self, request: BinaryMessage) -> BinaryMessage {
    //     match request.msg_type {
    //         1 => {  // 生产者请求
    //             let msg_id = self.broker.send_message("default_topic", request.payload);
    //             BinaryMessage::new_response(1, msg_id.unwrap_or(0))
    //         }
    //         2 => {  // 消费者请求
    //             let response = self.broker.fetch_message("default_topic", 0, request.msg_id);
    //             BinaryMessage::new_response(2, response.unwrap_or_else(|| vec![]))
    //         }
    //         _ => BinaryMessage::new_response(0, vec![]),
    //     }
    // }
}
