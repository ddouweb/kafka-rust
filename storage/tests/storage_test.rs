use storage::LogSegment;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let path = "test_log";
        let mut log = LogSegment::new(path).unwrap();

        let msg1 = b"Hello, Kafka!";
        let msg2 = b"Another message";

        log.append_message(msg1).unwrap();
        log.append_message(msg2).unwrap();

        let messages = log.read_messages().unwrap();
        assert_eq!(messages, vec![msg1.to_vec(), msg2.to_vec()]);

        //std::fs::remove_file(path).unwrap(); // 清理测试文件
    }
}
