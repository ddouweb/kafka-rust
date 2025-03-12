use std::io::{self, Read, Write};
#[derive(Debug)]
pub struct BinaryMessage {
    pub msg_type: u8,       // 消息类型
    pub msg_id: u32,        // 消息唯一标识
    pub payload: Vec<u8>,   // 消息内容
}

impl BinaryMessage {
    /// **将消息序列化成二进制格式**
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // 计算消息总长度
        let msg_length = 1 + 4 + self.payload.len();
        
        buffer.write_all(&(msg_length as u32).to_be_bytes()).unwrap(); // 4 字节 长度
        buffer.write_all(&self.msg_type.to_be_bytes()).unwrap(); // 1 字节 类型
        buffer.write_all(&self.msg_id.to_be_bytes()).unwrap();   // 4 字节 标识
        buffer.write_all(&self.payload).unwrap();               // 可变长度 数据

        buffer
    }

    /// **从二进制数据解析成 `BinaryMessage`**
    pub fn decode(stream: &mut impl Read) -> io::Result<Self> {
        let mut length_buf: [u8; 4] = [0u8; 4]; // 读取 4 字节长度
        stream.read_exact(&mut length_buf)?;
        let length: usize = u32::from_be_bytes(length_buf) as usize;

        let mut msg_type_buf = [0u8; 1]; // 读取 1 字节消息类型
        stream.read_exact(&mut msg_type_buf)?;
        let msg_type = msg_type_buf[0];

        let mut msg_id_buf = [0u8; 4]; // 读取 4 字节消息 ID
        stream.read_exact(&mut msg_id_buf)?;
        let msg_id = u32::from_be_bytes(msg_id_buf);

        let mut payload = vec![0u8; length - 5]; // 读取剩余字节作为消息体
        stream.read_exact(&mut payload)?;

        Ok(BinaryMessage {
            msg_type,
            msg_id,
            payload,
        })
    }
}
