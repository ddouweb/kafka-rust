use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct LogSegment {
    file: File,
}

impl LogSegment {
    /// 创建或打开一个日志段文件
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(Path::new(path))?;
        Ok(Self { file })
    }

    /// 追加消息到日志段
    pub fn append_message(&mut self, message: &[u8]) -> io::Result<()> {
        let length = (message.len() as u32).to_be_bytes(); // 4 字节消息长度
        self.file.write_all(&length)?;
        self.file.write_all(message)?;
        self.file.flush()?; // 确保数据写入磁盘
        Ok(())
    }

    /// 读取所有消息
    pub fn read_messages(&mut self) -> io::Result<Vec<Vec<u8>>> {
        let mut messages = Vec::new();
        self.file.seek(SeekFrom::Start(0))?;
        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer)?;

        let mut pos = 0;
        while pos < buffer.len() {
            if pos + 4 > buffer.len() {
                break;
            }
            let length = u32::from_be_bytes(buffer[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            if pos + length > buffer.len() {
                break;
            }
            messages.push(buffer[pos..pos + length].to_vec());
            pos += length;
        }
        Ok(messages)
    }
}
