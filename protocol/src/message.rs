use std::io::{self, Read, Write};
//use serde::{Serialize, Deserialize};

/// 消息类型枚举，对应不同的协议操作
#[derive(Debug, Clone, Copy, PartialEq, Eq, /* Serialize, Deserialize */)]
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
    pub msg_type: MessageType,  // 消息类型
    pub msg_id: u32,           // 消息唯一标识
    //pub version: u8,           // 协议版本
    pub payload: Vec<u8>,      // 消息内容
}

impl BinaryMessage {
    /// 创建新的二进制消息
    pub fn new(msg_type: MessageType, msg_id: u32, /* version: u8, */ payload: Vec<u8>) -> Self {
        Self {
            msg_type,
            msg_id,
            // version: version,
            payload,
        }
    }

    /// 将消息序列化成二进制格式
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // 计算消息总长度 (1字节类型 + 4字节ID + 1字节版本 + payload长度)
        let msg_length = 1 + 4 /* + 1 */ + self.payload.len();

        buffer.write_all(&(msg_length as u32).to_be_bytes()).unwrap(); // 4字节 长度
        buffer.write_all(&[self.msg_type.into()]).unwrap();           // 1字节 类型
        buffer.write_all(&self.msg_id.to_be_bytes()).unwrap();        // 4字节 标识
        //buffer.write_all(&[self.version]).unwrap();                   // 1字节 版本
        buffer.write_all(&self.payload).unwrap();                     // 可变长度 数据
        buffer
    }

    /// 从二进制数据解析成 BinaryMessage
    pub fn decode(buffer: &[u8]) -> io::Result<Self> {
        // 确保 buffer 至少有 5 字节（1字节类型 + 4字节ID /* + 1字节版本 */）
        if buffer.len() < 5 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer too short",
            ));
        }

        // 读取消息类型（1字节）
        let msg_type = MessageType::from(buffer[0]);
        let body = &buffer[1..];

        // 读取消息 ID（4字节）
        let msg_id = u32::from_be_bytes(body[..4].try_into().unwrap());
        //let version = body[4];

        // 剩下的就是 payload
        let payload = &body[4..];

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            //version,
            payload: payload.to_vec(),
        })
    }

    /// 从数据流解析成 BinaryMessage
    pub fn decode_message(stream: &mut impl Read) -> io::Result<Self> {
        let mut msg_type_buf = [0u8; 1];// 读取 1 字节消息类型
        stream.read_exact(&mut msg_type_buf)?;
        let msg_type = MessageType::from(msg_type_buf[0]);

        let mut msg_id_buf = [0u8; 4];
        stream.read_exact(&mut msg_id_buf)?;
        let msg_id = u32::from_be_bytes(msg_id_buf);

        //let mut version_buf = [0u8; 1];
        //stream.read_exact(&mut version_buf)?;
        //let version = version_buf[0];

        let mut payload = Vec::new();
        stream.read_to_end(&mut payload)?;

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            //version,
            payload,
        })
    }
} 