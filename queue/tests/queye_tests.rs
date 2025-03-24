use queue::LogQueue;
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_LOG_DIR: &str = "test_log_queue";

    /// **测试初始化**
    fn setup() {
        let _ = fs::remove_dir_all(TEST_LOG_DIR); // 清理旧数据
        let _ = fs::create_dir(TEST_LOG_DIR);
    }

    #[test]
    fn test_append_message() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 1024).expect("Failed to create LogQueue");
        let message = b"hello kafka";
        let offset = queue
            .append_message(message)
            .expect("Failed to append message");
        assert_eq!(offset, 0, "First message should be at offset 0");
    }

    #[test]
    fn test_read_message() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 1024).expect("Failed to create LogQueue");
        let message = b"hello kafka";
        let offset = queue
            .append_message(message)
            .expect("Failed to append message");

        let read_msg: Option<Vec<u8>> = queue.read_message(offset).expect("Failed to read message");
        assert_eq!(
            read_msg,
            Some(message.to_vec()),
            "Read message should match written message"
        );
    }

    #[test]
    fn test_segment_rotation() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 10).expect("Failed to create LogQueue"); // 设定极小的 segment 触发滚动
        queue.append_message(b"1234567890").unwrap();
        queue.append_message(b"new segment").unwrap();
        queue.append_message(b"987654321").unwrap();
    }

    #[test]
    fn test_write_times() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 1024).expect("Failed to create LogQueue");
        for offset in 0..3000 {
            // 将 offset 转换为字节并附加到消息中
            let message = format!("heool kafka {}", offset).into_bytes();
            match queue.append_message(&message) {
                Ok(write_offset) => {
                    assert_eq!(offset, write_offset, "message offset is error");
                    let read_message = queue.read_message(offset).unwrap();
                    // 验证读取的消息是否与写入的消息一致
                    assert_eq!(
                        read_message,
                        Some(message),
                        "Read message does not match written message"
                    );
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };
        }
    }
}
