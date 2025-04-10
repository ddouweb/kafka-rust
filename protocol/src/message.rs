use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use crate::{ClientRequest, GetClusterInfoRequest};

/// 消息类型枚举，对应不同的协议操作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Produce = 0,
    Fetch = 1,
    Metadata = 2,
    OffsetFetch = 3,
    JoinGroup = 4,
    SyncGroup = 5,
    CreateTopic = 6,
    DeleteTopic = 7,
    DescribeTopic = 8,
    ListTopics = 9,
    UpdateTopicConfig = 10,
    GetClusterInfo = 11,
    Heartbeat = 12,
    LeaveGroup = 13,
    Unknown = 255,
}

impl Hash for MessageType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Produce,
            1 => MessageType::Fetch,
            2 => MessageType::Metadata,
            3 => MessageType::OffsetFetch,
            4 => MessageType::JoinGroup,
            5 => MessageType::SyncGroup,
            6 => MessageType::CreateTopic,
            7 => MessageType::DeleteTopic,
            8 => MessageType::DescribeTopic,
            9 => MessageType::ListTopics,
            10 => MessageType::UpdateTopicConfig,
            11 => MessageType::GetClusterInfo,
            12 => MessageType::Heartbeat,
            13 => MessageType::LeaveGroup,
            _ => MessageType::Unknown,
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        value as u8
    }
}

/// 二进制消息结构，用于网络传输
#[derive(Debug, Clone)]
pub struct BinaryMessage {
    pub msg_type: MessageType,      // 消息类型
    pub msg_id: u32,               // 消息唯一标识
    pub correlation_id: u32,       // 请求-响应关联ID
    pub client_id: u32,         // 客户端ID
    pub payload: Vec<u8>,          // 消息内容
}

impl BinaryMessage {
    /// 创建新的二进制消息
    pub fn new(
        msg_type: MessageType,
        msg_id: u32,
        correlation_id: u32,
        client_id: u32,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            msg_type,
            msg_id,
            correlation_id,
            client_id,
            payload,
        }
    }

    /// 从 ClientRequest 创建 BinaryMessage
    pub fn from_request(request: &ClientRequest, msg_id: u32, correlation_id: u32, client_id: u32) -> Self {
        let (msg_type, payload) = match request {
            ClientRequest::Produce(req) => (MessageType::Produce, serde_json::to_vec(req).unwrap()),
            ClientRequest::Fetch(req) => (MessageType::Fetch, serde_json::to_vec(req).unwrap()),
            ClientRequest::Metadata(req) => (MessageType::Metadata, serde_json::to_vec(req).unwrap()),
            ClientRequest::OffsetFetch(req) => (MessageType::OffsetFetch, serde_json::to_vec(req).unwrap()),
            ClientRequest::JoinGroup(req) => (MessageType::JoinGroup, serde_json::to_vec(req).unwrap()),
            ClientRequest::SyncGroup(req) => (MessageType::SyncGroup, serde_json::to_vec(req).unwrap()),
            ClientRequest::CreateTopic(req) => (MessageType::CreateTopic, serde_json::to_vec(req).unwrap()),
            ClientRequest::DeleteTopic(req) => (MessageType::DeleteTopic, serde_json::to_vec(req).unwrap()),
            ClientRequest::DescribeTopic(req) => (MessageType::DescribeTopic, serde_json::to_vec(req).unwrap()),
            ClientRequest::ListTopics(req) => (MessageType::ListTopics, serde_json::to_vec(req).unwrap()),
            ClientRequest::UpdateTopicConfig(req) => (MessageType::UpdateTopicConfig, serde_json::to_vec(req).unwrap()),
            ClientRequest::GetClusterInfo(_) => (MessageType::GetClusterInfo, vec![]), // 空结构体直接返回空数组
        };
        Self::new(msg_type, msg_id, correlation_id, client_id, payload)
    }

    /// 将 BinaryMessage 转换为 ClientRequest
    pub fn to_request(&self) -> io::Result<ClientRequest> {
        match self.msg_type {
            MessageType::Produce => Ok(ClientRequest::Produce(serde_json::from_slice(&self.payload)?)),
            MessageType::Fetch => Ok(ClientRequest::Fetch(serde_json::from_slice(&self.payload)?)),
            MessageType::Metadata => Ok(ClientRequest::Metadata(serde_json::from_slice(&self.payload)?)),
            MessageType::OffsetFetch => Ok(ClientRequest::OffsetFetch(serde_json::from_slice(&self.payload)?)),
            MessageType::JoinGroup => Ok(ClientRequest::JoinGroup(serde_json::from_slice(&self.payload)?)),
            MessageType::SyncGroup => Ok(ClientRequest::SyncGroup(serde_json::from_slice(&self.payload)?)),
            MessageType::CreateTopic => Ok(ClientRequest::CreateTopic(serde_json::from_slice(&self.payload)?)),
            MessageType::DeleteTopic => Ok(ClientRequest::DeleteTopic(serde_json::from_slice(&self.payload)?)),
            MessageType::DescribeTopic => Ok(ClientRequest::DescribeTopic(serde_json::from_slice(&self.payload)?)),
            MessageType::ListTopics => Ok(ClientRequest::ListTopics(serde_json::from_slice(&self.payload)?)),
            MessageType::UpdateTopicConfig => Ok(ClientRequest::UpdateTopicConfig(serde_json::from_slice(&self.payload)?)),
            MessageType::GetClusterInfo => Ok(ClientRequest::GetClusterInfo(GetClusterInfoRequest {})), // 直接创建空结构体
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown message type")),
        }
    }

    /// 将消息序列化成二进制格式
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // 计算消息总长度 (1字节类型 + 4字节ID + 4字节correlation_id + 4字节client_id + payload长度)
        let msg_length = 1 + 4 + 4 + 4 + self.payload.len();

        buffer.write_all(&(msg_length as u32).to_be_bytes()).unwrap(); // 4字节 长度
        buffer.write_all(&[self.msg_type.into()]).unwrap();           // 1字节 类型
        buffer.write_all(&self.msg_id.to_be_bytes()).unwrap();        // 4字节 标识
        buffer.write_all(&self.correlation_id.to_be_bytes()).unwrap(); // 4字节 correlation_id
        buffer.write_all(&self.client_id.to_be_bytes()).unwrap();        // 4字节 client_id
        buffer.write_all(&self.payload).unwrap();                     // 可变长度 数据
        buffer
    }

    /// 从二进制数据解析成 BinaryMessage
    pub fn decode(buffer: &[u8]) -> io::Result<Self> {
        // 确保 buffer 至少有 17 字节（4字节消息长度 + 13字节消息体）
        if buffer.len() < 17 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer too short",
            ));
        }

        // 读取消息长度（4字节）
        let msg_length = u32::from_be_bytes(buffer[..4].try_into().unwrap()) as usize;
        if buffer.len() < msg_length + 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer length mismatch",
            ));
        }

        // 读取消息体（从第5字节开始）
        let body = &buffer[4..];

        // 读取消息类型（1字节）
        let msg_type = MessageType::from(body[0]);
        let body = &body[1..];

        // 读取消息 ID（4字节）
        let msg_id = u32::from_be_bytes(body[..4].try_into().unwrap());
        let body = &body[4..];

        // 读取 correlation_id（4字节）
        let correlation_id = u32::from_be_bytes(body[..4].try_into().unwrap());
        let body = &body[4..];

        // 读取 client_id（4字节）
        let client_id = u32::from_be_bytes(body[..4].try_into().unwrap());
        let body = &body[4..];

        // 剩下的就是 payload
        let payload = body.to_vec();

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            correlation_id,
            client_id,
            payload,
        })
    }

    /// 从数据流解析成 BinaryMessage
    pub fn decode_message(stream: &mut impl Read) -> io::Result<Self> {
        // 读取消息长度（4字节）
        let mut length_buf = [0u8; 4];
        stream.read_exact(&mut length_buf)?;
        let msg_length = u32::from_be_bytes(length_buf) as usize;

        // 读取消息类型（1字节）
        let mut msg_type_buf = [0u8; 1];
        stream.read_exact(&mut msg_type_buf)?;
        let msg_type = MessageType::from(msg_type_buf[0]);

        // 读取消息 ID（4字节）
        let mut msg_id_buf = [0u8; 4];
        stream.read_exact(&mut msg_id_buf)?;
        let msg_id = u32::from_be_bytes(msg_id_buf);

        // 读取 correlation_id（4字节）
        let mut correlation_id_buf = [0u8; 4];
        stream.read_exact(&mut correlation_id_buf)?;
        let correlation_id = u32::from_be_bytes(correlation_id_buf);

        // 读取 client_id（4字节）
        let mut client_id_buf = [0u8; 4];
        stream.read_exact(&mut client_id_buf)?;
        let client_id = u32::from_be_bytes(client_id_buf);

        // 读取 payload（剩余字节）
        let mut payload = vec![0u8; msg_length - 13]; // 13 = 1(msg_type) + 4(msg_id) + 4(correlation_id) + 4(client_id)
        stream.read_exact(&mut payload)?;

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            correlation_id,
            client_id,
            payload,
        })
    }
} 