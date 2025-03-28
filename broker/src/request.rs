use network::BinaryMessage;
use crate::broker::Broker;

pub struct RequestHandler {
    broker: Broker,
}

impl RequestHandler {
    pub fn new(broker: Broker) -> Self {
        Self { broker }
    }

    pub fn handle_request(&self, request: BinaryMessage) -> BinaryMessage {
        match request.msg_type {
            1 => {  // 生产者请求
                let msg_id = self.broker.send_message("default_topic", request.payload);
                BinaryMessage {
                    msg_type: 1,
                    msg_id: msg_id.unwrap_or(0) as u32,
                    payload: vec![],
                }
            }
            2 => {  // 消费者请求
                let response = self.broker.fetch_message("default_topic", 0, request.msg_id);
                BinaryMessage {
                    msg_type: 2,
                    msg_id: request.msg_id,
                    payload: response.unwrap_or_else(|_| None).unwrap_or_else(|| vec![]),
                }
            }
            _ => BinaryMessage {
                msg_type: 0,
                msg_id: 0,
                payload: vec![],
            },
        }
    }
}
