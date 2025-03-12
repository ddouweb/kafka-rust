use network::message::BinaryMessage;
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_binary_message_encode_decode() {
        let original_msg = BinaryMessage {
            msg_type: 1,
            msg_id: 42,
            payload: b"Hello, Kafka!".to_vec(),
        };

        let encoded = original_msg.encode();
        let mut cursor = Cursor::new(encoded);

        let decoded_msg = BinaryMessage::decode(&mut cursor).unwrap();

        assert_eq!(original_msg.msg_type, decoded_msg.msg_type);
        assert_eq!(original_msg.msg_id, decoded_msg.msg_id);
        assert_eq!(original_msg.payload, decoded_msg.payload);
    }
}
