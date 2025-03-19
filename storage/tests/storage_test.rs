use storage::LogSegment;
#[cfg(test)]
mod tests {

    use storage::retention;

    use super::*;

    #[test]
    fn test(){
        let mut log: LogSegment = LogSegment::new("logs", 0, 100).unwrap();    
        for _ in 1..300  {
            test_write_times(&mut log);
        }
    }
    fn test_write_times(log: &mut LogSegment) {
        

        let msg1 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1";
        let msg2 = b"QWERTYUIOPASDFGHJKLZXCVBNM22";
        let msg3 = b"1234567890123456789012345678XXX";

        // let msg1 = b"123";
        // let msg2 = b"456";
        // let msg3 = b"789";

        let offset1 = log.append_message(msg1).unwrap();
        let offset2 = log.append_message(msg2).unwrap();
        let offset3 = log.append_message(msg3).unwrap();

        println!("offset1: {}", offset1);
        println!("offset2: {}", offset2);
        println!("offset3: {}", offset3);

        let message1 = log.read_message(offset1).unwrap();

        //assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(offset2).unwrap();
        //assert_eq!(message2, Some(msg2.to_vec()));

        let message3 = log.read_message(offset3).unwrap();
        //assert_eq!(message3, Some(msg3.to_vec()));

    }

    fn test_storage_delete(mut log: LogSegment) {
        let msg1 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1";
        let msg2 = b"QWERTYUIOPASDFGHJKLZXCVBNM22";
        let msg3 = b"1234567890123456789012345678XXX";

        let offset1 = log.append_message(msg1).unwrap();
        let offset2 = log.append_message(msg2).unwrap();
        let offset3 = log.append_message(msg3).unwrap();

        println!("offset4: {}", offset1);
        println!("offset5: {}", offset2);
        println!("offset6: {}", offset3);

        let message1 = log.read_message(offset1).unwrap();

        assert_eq!(message1, Some(msg1.to_vec()));

        let message2 = log.read_message(offset2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));

        let message3 = log.read_message(offset3).unwrap();
        assert_eq!(message3, Some(msg3.to_vec()));

        //std::fs::remove_file("log.log").unwrap();
        //std::fs::remove_file("log.index").unwrap();
        test_storage_read_all_messages();
    }

    fn test_storage_read_all_messages() {
        let mut segment = LogSegment::new("logs1/", 0, 1024).unwrap();
        segment.append_message(b"Hello, Kafka!").unwrap();

        if let Some(msg) = segment.read_message(0).unwrap() {
            println!("Read message: {:?}", String::from_utf8_lossy(&msg));
        }
        retention::clean_old_segments("logs1/".to_string());
    }
}
