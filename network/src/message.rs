use std::io::{self, Read, Write};
#[derive(Debug)]
pub struct BinaryMessage {
    pub msg_type: u8,     // 消息类型
    pub msg_id: u32,      // 消息唯一标识
    pub payload: Vec<u8>, // 消息内容
}

impl BinaryMessage {
    /// **将消息序列化成二进制格式**
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // 计算消息总长度
        let msg_length = 1 + 4 + self.payload.len();

        buffer.write_all(&(msg_length as u32).to_be_bytes()).unwrap(); // 4 字节 长度
        buffer.write_all(&self.msg_type.to_be_bytes()).unwrap(); // 1 字节 类型
        buffer.write_all(&self.msg_id.to_be_bytes()).unwrap(); // 4 字节 标识
        buffer.write_all(&self.payload).unwrap(); // 可变长度 数据
        buffer
    }

    /// **从BinaryMessage的二进制数据解析成 `BinaryMessage`**
    pub fn decode(buffer: &[u8]) -> io::Result<Self> {
        // 确保 buffer 至少有 5 字节（1 字节类型 + 4 字节 ID）
        if buffer.len() < 5 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer too short",
            ));
        }

        // 读取消息类型（1 字节）
        let msg_type = buffer[0];
        let body = &buffer[1..];

        // 读取消息 ID（4 字节）
        let msg_id = u32::from_be_bytes(body[..4].try_into().unwrap());
        let payload = &body[4..];

        // 剩下的就是 payload
        let payload = payload.to_vec();

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            payload,
        })
    }


    /// **从处理好的BinaryMessage数据流解析成 `BinaryMessage`**
    pub fn decode_message(stream: &mut impl Read) -> io::Result<Self> {
        let mut msg_type_buf = [0u8; 1]; // 读取 1 字节消息类型
        stream.read_exact(&mut msg_type_buf)?;
        let msg_type = msg_type_buf[0];

        let mut msg_id_buf = [0u8; 4]; // 读取 4 字节消息 ID
        stream.read_exact(&mut msg_id_buf)?;
        let msg_id = u32::from_be_bytes(msg_id_buf);

        let mut payload = Vec::new();
        stream.read_exact(&mut payload)?;

        println!(
            "decode_message message: msg_type={}, msg_id={}, payload={:?}",
            msg_type, msg_id, payload
        );

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            payload,
        })
    }
}
