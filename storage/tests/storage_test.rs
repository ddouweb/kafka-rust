use storage::LogSegment;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_storage() {
        let path = "log";
        let mut log: LogSegment = LogSegment::new(path,  10000).unwrap();

        let msg1 = "Hello, Kafka!".as_bytes();
        let msg2 = b"Another message";

        let msg_no1 = log.append_message(msg1).unwrap();
        let msg_no2 = log.append_message(msg2).unwrap();

        println!("msg_no1: {}",msg_no1);
        println!("msg_no2: {}",msg_no2);

        let message1 = log.read_message(msg_no1).unwrap();
        
        assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(msg_no2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));
        test_storage_delete(log);
    }

    fn test_storage_delete(mut log: LogSegment) {
    
        let msg1 = "Hello, Kafka2!".as_bytes();
        let msg2 = b"Another message2";

        let msg_no1 = log.append_message(msg1).unwrap();
        let msg_no2 = log.append_message(msg2).unwrap();

        println!("msg_no1: {}",msg_no1);
        println!("msg_no2: {}",msg_no2);

        let message1 = log.read_message(msg_no1).unwrap();
        
        assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(msg_no2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));



        //let all_messages = log.read_all_messages().unwrap();
        //assert_eq!(all_messages, vec![msg1.to_vec(), msg2.to_vec()]);

        //log.clean_files();
        //std::fs::remove_file("log.log").unwrap();
        //std::fs::remove_file("log.index").unwrap();
    }
}
