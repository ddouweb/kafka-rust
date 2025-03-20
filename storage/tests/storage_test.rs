use storage::LogSegment;
#[cfg(test)]
mod tests {

    use storage::retention;

    use super::*;

    #[test]
    fn test() {
        let mut log: LogSegment = LogSegment::new("logs", 0, 1024 * 1024).unwrap();
        test_read_write_times(&mut log);
        for _ in 1..300 {
            test_read_write_times(&mut log);
        }
        for _ in 1..300 {
            test_write_times(&mut log);
        }
        for offset in 0..601 {
            test_read_times(&mut log, offset);
        }
        test_storage_read_all_messages();
        test_id();
    }
    fn test_id() {
        for _ in 1..10 {
            let id = LogSegment::get_next_offset();
            eprintln!("id:{}", id);
        }
    }
    fn test_read_times(log: &mut LogSegment, offset: u64) {
        let message = log.read_message(offset).unwrap();
        eprintln!(
            "offset:{} is : {:?}",
            offset,
            option_vec_u8_to_string(message)
        );
    }

    fn option_vec_u8_to_string(input: Option<Vec<u8>>) -> Option<String> {
        input.map(|vec| String::from_utf8_lossy(&vec).to_string())
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

        eprintln!("offset1: {}", offset1);
        eprintln!("offset2: {}", offset2);
        eprintln!("offset3: {}", offset3);
    }
    fn test_read_write_times(log: &mut LogSegment) {
        let msg1 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1";
        let msg2 = b"QWERTYUIOPASDFGHJKLZXCVBNM22";
        let msg3 = b"1234567890123456789012345678XXX";

        // let msg1 = b"123";
        // let msg2 = b"456";
        // let msg3 = b"789";

        let offset1 = log.append_message(msg1).unwrap();
        eprintln!("offset1: {}", offset1);
        let message1 = log.read_message(offset1).unwrap();
        assert_eq!(message1, Some(msg1.to_vec()));

        let offset2 = log.append_message(msg2).unwrap();
        eprintln!("offset2: {}", offset2);
        let message2 = log.read_message(offset2).unwrap();
        assert_eq!(message2, Some(msg2.to_vec()));

        let offset3 = log.append_message(msg3).unwrap();
        eprintln!("offset3: {}", offset3);
        let message3 = log.read_message(offset3).unwrap();
        assert_eq!(message3, Some(msg3.to_vec()));
    }

    fn test_storage_read_all_messages() {
        retention::clean_old_segments("logs".to_string());
    }
}
