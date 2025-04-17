use protocol::message::MessageType;
use protocol::message::BinaryMessage;
use protocol::request::ClientRequest;
use protocol::request::ProduceRequest;
use protocol::request::GetClusterInfoRequest;
use protocol::request::FetchRequest;
#[test]
fn test_message_type_conversion() {
    // 测试 MessageType 到 u8 的转换
    assert_eq!(u8::from(MessageType::Produce), 0);
    assert_eq!(u8::from(MessageType::Fetch), 1);
    assert_eq!(u8::from(MessageType::Metadata), 2);
    assert_eq!(u8::from(MessageType::OffsetFetch), 3);
    assert_eq!(u8::from(MessageType::JoinGroup), 4);
    assert_eq!(u8::from(MessageType::SyncGroup), 5);
    assert_eq!(u8::from(MessageType::CreateTopic), 6);
    assert_eq!(u8::from(MessageType::DeleteTopic), 7);
    assert_eq!(u8::from(MessageType::DescribeTopic), 8);
    assert_eq!(u8::from(MessageType::ListTopics), 9);
    assert_eq!(u8::from(MessageType::UpdateTopicConfig), 10);
    assert_eq!(u8::from(MessageType::GetClusterInfo), 11);
    assert_eq!(u8::from(MessageType::Heartbeat), 12);
    assert_eq!(u8::from(MessageType::LeaveGroup), 13);
    assert_eq!(u8::from(MessageType::Unknown), 255);

    // 测试 u8 到 MessageType 的转换
    assert_eq!(MessageType::from(0), MessageType::Produce);
    assert_eq!(MessageType::from(1), MessageType::Fetch);
    assert_eq!(MessageType::from(2), MessageType::Metadata);
    assert_eq!(MessageType::from(3), MessageType::OffsetFetch);
    assert_eq!(MessageType::from(4), MessageType::JoinGroup);
    assert_eq!(MessageType::from(5), MessageType::SyncGroup);
    assert_eq!(MessageType::from(6), MessageType::CreateTopic);
    assert_eq!(MessageType::from(7), MessageType::DeleteTopic);
    assert_eq!(MessageType::from(8), MessageType::DescribeTopic);
    assert_eq!(MessageType::from(9), MessageType::ListTopics);
    assert_eq!(MessageType::from(10), MessageType::UpdateTopicConfig);
    assert_eq!(MessageType::from(11), MessageType::GetClusterInfo);
    assert_eq!(MessageType::from(12), MessageType::Heartbeat);
    assert_eq!(MessageType::from(13), MessageType::LeaveGroup);
    assert_eq!(MessageType::from(255), MessageType::Unknown);
}

#[test]
fn test_binary_message_creation() {
    let msg = BinaryMessage::new(
        MessageType::Produce,
        123,
        456,
        789,
        vec![1, 2, 3, 4],
    );

    assert_eq!(msg.msg_type, MessageType::Produce);
    assert_eq!(msg.msg_id, 123);
    assert_eq!(msg.correlation_id, 456);
    assert_eq!(msg.client_id, 789);
    assert_eq!(msg.payload, vec![1, 2, 3, 4]);
}

#[test]
fn test_client_request_to_binary_message() {
    // 创建 ProduceRequest
    let produce_request = ClientRequest::Produce(ProduceRequest {
        topic: "test-topic".to_string(),
        partition: 0,
        messages: vec![1, 2, 3, 4],
    });

    // 转换为 BinaryMessage
    let binary_msg = BinaryMessage::from_request(
        &produce_request,
        123,
        456,
        789,
    );

    assert_eq!(binary_msg.msg_type, MessageType::Produce);
    assert_eq!(binary_msg.msg_id, 123);
    assert_eq!(binary_msg.correlation_id, 456);
    assert_eq!(binary_msg.client_id, 789);

    // 转换回 ClientRequest
    let decoded_request = binary_msg.to_request().unwrap();
    match decoded_request {
        ClientRequest::Produce(req) => {
            assert_eq!(req.topic, "test-topic");
            assert_eq!(req.partition, 0);
            assert_eq!(req.messages, vec![1, 2, 3, 4]);
        }
        _ => panic!("Expected ProduceRequest"),
    }
}

#[test]
fn test_fetch_request_to_binary_message() {
    // 创建 FetchRequest
    let fetch_request = ClientRequest::Fetch(FetchRequest {
        topic: "test-topic".to_string(),
        partition: 0,
        offset: 100,
        max_bytes: 1024,
    });

    // 转换为 BinaryMessage
    let binary_msg = BinaryMessage::from_request(
        &fetch_request,
        123,
        456,
        789,
    );

    assert_eq!(binary_msg.msg_type, MessageType::Fetch);
    assert_eq!(binary_msg.msg_id, 123);
    assert_eq!(binary_msg.correlation_id, 456);
    assert_eq!(binary_msg.client_id, 789);

    // 转换回 ClientRequest
    let decoded_request = binary_msg.to_request().unwrap();
    match decoded_request {
        ClientRequest::Fetch(req) => {
            assert_eq!(req.topic, "test-topic");
            assert_eq!(req.partition, 0);
            assert_eq!(req.offset, 100);
        }
        _ => panic!("Expected FetchRequest"),
    }
}

#[test]
fn test_invalid_message_type() {
    // 创建一个未知类型的消息
    let invalid_msg = BinaryMessage::new(
        MessageType::Unknown,
        123,
        456,
        789,
        vec![1, 2, 3, 4],
    );

    // 尝试转换为 ClientRequest 应该失败
    assert!(invalid_msg.to_request().is_err());
}


#[test]
fn test_valid_payload() {

    // 创建 ProduceRequest
    let produce_request = ClientRequest::Produce(ProduceRequest {
        topic: "test-topic".to_string(),
        partition: 0,
        messages: vec![1, 2, 3, 4],
    });

    // 转换为 BinaryMessage
    let binary_msg = BinaryMessage::from_request(
        &produce_request,
        123,
        456,
        789,
    );

    // 有效负载 尝试转换为 ClientRequest 应该成功
    assert!(binary_msg.to_request().is_ok());
}

#[test]
fn test_invalid_payload() {

    // 创建一个带有无效负载的消息
    let msg = BinaryMessage::new(
        MessageType::Produce,  // 使用需要负载的消息类型
        123,
        456,
        789,
        vec![1,2,3,4], // 无效负载
    );

    // 无效负载 尝试转换为 ClientRequest 应该失败
    assert!(msg.to_request().is_err());


    // 创建一个带有无效负载的消息 GetClusterInfo 本身不处理负载，所以可以成功。如果传递负载，会丢弃负载信息
    let invalid_msg = BinaryMessage::new(
        MessageType::GetClusterInfo,  // 使用不需要负载的消息类型
        123,
        456,
        789,
        vec![], // 有负载也会被忽略
    );

    // 尝试转换为 ClientRequest 应该成功
    assert!(invalid_msg.to_request().is_ok());
}

#[test]
fn test_get_cluster_info_message() {
    // 创建 GetClusterInfo 请求
    let cluster_info_request = ClientRequest::GetClusterInfo(GetClusterInfoRequest {});

    // 转换为 BinaryMessage
    let binary_msg = BinaryMessage::from_request(
        &cluster_info_request,
        123,
        456,
        789,
    );

    assert_eq!(binary_msg.msg_type, MessageType::GetClusterInfo);
    assert_eq!(binary_msg.msg_id, 123);
    assert_eq!(binary_msg.correlation_id, 456);
    assert_eq!(binary_msg.client_id, 789);
    // 空结构体序列化后应该是一个空数组
    assert_eq!(binary_msg.payload, Vec::<u8>::new());

    // 转换回 ClientRequest
    let decoded_request = binary_msg.to_request().unwrap();
    match decoded_request {
        ClientRequest::GetClusterInfo(_) => (), // 成功转换为 GetClusterInfo 请求
        _ => panic!("Expected GetClusterInfoRequest"),
    }
}

#[test]
fn test_binary_message_encode_decode() {
    let original_msg = BinaryMessage::new(
        MessageType::Produce,
        123,
        456,
        789,
        vec![1, 2, 3, 4],
    );

    // 编码
    let encoded = original_msg.encode();
    assert!(encoded.len() >= 1 + 4 + 4 + 4);

    // 解码
    let decoded_msg = BinaryMessage::decode(&encoded).unwrap();
    assert_eq!(decoded_msg.msg_type, original_msg.msg_type);
    assert_eq!(decoded_msg.msg_id, original_msg.msg_id);
    assert_eq!(decoded_msg.correlation_id, original_msg.correlation_id);
    assert_eq!(decoded_msg.client_id, original_msg.client_id);
    assert_eq!(decoded_msg.payload, original_msg.payload);
}

#[test]
fn test_message_stream_decode() {
    use std::io::Cursor;

    // 创建一个测试消息
    let original_msg = BinaryMessage::new(
        MessageType::Produce,
        123,
        456,
        789,
        vec![1, 2, 3, 4],
    );

    // 编码消息
    let encoded = original_msg.encode();

    // 创建内存流
    let mut stream = Cursor::new(encoded);

    // 从流中解码
    let decoded_msg = BinaryMessage::decode_message(&mut stream).unwrap();

    // 验证解码结果
    assert_eq!(decoded_msg.msg_type, original_msg.msg_type);
    assert_eq!(decoded_msg.msg_id, original_msg.msg_id);
    assert_eq!(decoded_msg.correlation_id, original_msg.correlation_id);
    assert_eq!(decoded_msg.client_id, original_msg.client_id);
    assert_eq!(decoded_msg.payload, original_msg.payload);
} 