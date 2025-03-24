use storage::LogSegment;
use storage::io_result::IoResult;
#[cfg(test)]
mod tests {

    use storage::retention;

    use super::*;

    const TEST_LOG_DIR: &str = "test_log_queue";

    /// **测试初始化**
    fn setup() {
        let _ = std::fs::remove_dir_all(TEST_LOG_DIR); // 清理旧数据
        let _ = std::fs::create_dir(TEST_LOG_DIR);
    }

    #[test]
    fn test_offset() {
        setup();
        let mut log: LogSegment = LogSegment::new(TEST_LOG_DIR, 0, 1024 * 1024).unwrap();
        for except_offset in 0..3 {
            match log.append_message(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1") {
                Ok(IoResult::Success(offset)) => {
                    assert_eq!(offset, except_offset, "message offset is error");
                },
                Ok(IoResult::SegmentFull) => {
                    println!("SegmentFull");
                },
                Err(e) =>{
                    panic!("Error: {}, {}",e.kind(),e.to_string());
                }
            };
        }

        let mut newlog: LogSegment = LogSegment::new(TEST_LOG_DIR, 0, 1024 * 1024).unwrap();
        for new_except_offset in 3..100 {
            match newlog.append_message(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1") {
                Ok(IoResult::Success(new_offset)) => {
                    assert_eq!(new_offset, new_except_offset, "message offset is error");
                },
                Ok(IoResult::SegmentFull) => {
                    println!("SegmentFull");
                },
                Err(e) =>{
                    panic!("Error: {}, {}",e.kind(),e.to_string());
                }
            };
        }

    }
    
    #[test]
    fn test() {
        setup();
        let mut log: LogSegment = LogSegment::new(TEST_LOG_DIR, 0, 1024 * 1024).unwrap();
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

        log.append_message(msg1).unwrap();
        log.append_message(msg2).unwrap();
        log.append_message(msg3).unwrap();
    }
    fn test_read_write_times(log: &mut LogSegment) {
        let msg1 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1";
        let msg2 = b"QWERTYUIOPASDFGHJKLZXCVBNM22";
        let msg3 = b"1234567890123456789012345678XXX";

        match log.append_message(msg1) {
            Ok(IoResult::Success(offset)) => {
                let message = log.read_message(offset).unwrap();
                assert_eq!(message, Some(msg1.to_vec()));
            },
            Ok(IoResult::SegmentFull) => {
                println!("SegmentFull");
            },
            Err(e) =>{
                panic!("Error: {}, {}",e.kind(),e.to_string());
            }
        };

        match log.append_message(msg2) {
            Ok(IoResult::Success(offset)) => {
                let message = log.read_message(offset).unwrap();
                assert_eq!(message, Some(msg2.to_vec()));
            },
            Ok(IoResult::SegmentFull) => {
                println!("SegmentFull");
            },
            Err(e) =>{
                panic!("Error: {}, {}",e.kind(),e.to_string());
            }
        };

        match log.append_message(msg3) {
            Ok(IoResult::Success(offset)) => {
                let message = log.read_message(offset).unwrap();
                assert_eq!(message, Some(msg3.to_vec()));
            },
            Ok(IoResult::SegmentFull) => {
                println!("SegmentFull");
            },
            Err(e) =>{
                panic!("Error: {}, {}",e.kind(),e.to_string());
            }
        };
        
    }

    fn test_storage_read_all_messages() {
        retention::clean_old_segments("logs".to_string());
    }
}
