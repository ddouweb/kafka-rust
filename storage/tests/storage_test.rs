use storage::LogSegment;
#[cfg(test)]
mod tests {

    use storage::retention;

    use super::*;

    #[test]
    fn test_write() {

        let mut log: LogSegment = LogSegment::new("logs/",0, 1024).unwrap();

        let msg1 = b"Hello, Kafka!";
        let msg2 = b"Another message";

        let offset1 = log.append_message(msg1).unwrap();
        let offset2 = log.append_message(msg2).unwrap();

        println!("offset1: {}", offset1);
        println!("offset2: {}", offset2);

        let message1 = log.read_message(offset1).unwrap();

        assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(offset2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));
        test_storage_delete(log);
    }

    fn test_storage_delete(mut log: LogSegment) {
        let msg1 = "Hello, Kafka2!".as_bytes();
        let msg2 = b"Another message2";

        let msg_no1 = log.append_message(msg1).unwrap();
        let msg_no2 = log.append_message(msg2).unwrap();

        println!("msg_no1: {}", msg_no1);
        println!("msg_no2: {}", msg_no2);

        let message1 = log.read_message(msg_no1).unwrap();

        assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(msg_no2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));

        //std::fs::remove_file("log.log").unwrap();
        //std::fs::remove_file("log.index").unwrap();
    }

    fn test_storage_read_all_messages() {
        let mut segment = LogSegment::new("logs/",0, 1024).unwrap();
        segment.append_message(b"Hello, Kafka!").unwrap();

        if let Some(msg) = segment.read_message(0).unwrap() {
            println!("Read message: {:?}", String::from_utf8_lossy(&msg));
        }
        retention::clean_old_segments("logs/".to_string());
    }
}
