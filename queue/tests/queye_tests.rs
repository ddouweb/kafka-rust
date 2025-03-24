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
        let offset = queue.append_message(message).expect("Failed to append message");
        assert_eq!(offset, 0, "First message should be at offset 0");
    }

    #[test]
    fn test_read_message() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 1024).expect("Failed to create LogQueue");
        let message = b"hello kafka";
        let offset = queue.append_message(message).expect("Failed to append message");

        let read_msg: Option<Vec<u8>> = queue.read_message(offset).expect("Failed to read message");
        assert_eq!(read_msg, Some(message.to_vec()), "Read message should match written message");
    }

    #[test]
    fn test_segment_rotation() {
        setup();
        let mut queue = LogQueue::new(TEST_LOG_DIR, 1024).expect("Failed to create LogQueue"); // 设定极小的 segment 触发滚动
        queue.append_message(b"1234567890").unwrap();
        queue.append_message(b"new segment").unwrap();
    }
}
