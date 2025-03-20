use crate::concurrency::MutexFile;
use crate::mmap::MmapIndex;
use std::io::{self, Read, Seek, SeekFrom, Write};

use super::{INDEX_ENTRY_SIZE, MSG_HEADER_SIZE, OFFSET_SIZE, POS_SIZE};
pub struct LogSegment {
    log_file: MutexFile,     // 存储实际消息数据
    index_file: MutexFile,   // 存储索引
    mmap_index: MmapIndex,   // 存储索引,使用mmap
    base_offset: u64,        // 当前段的起始 offset
    offset: u64,             // 下一个消息的 offset
    max_segment_size: usize, // 单个段的最大大小
}

impl LogSegment {
    pub fn new(log_dir: &str, base_offset: u64, max_segment_size: usize) -> io::Result<Self> {
        Self::with_offset(log_dir, base_offset, max_segment_size, None)
    }

    pub fn with_offset(
        log_dir: &str,
        base_offset: u64,
        max_segment_size: usize,
        offset: Option<u64>,
    ) -> io::Result<Self> {
        let log_dir = log_dir.trim_end_matches('/');
        if !std::path::Path::new(log_dir).exists() {
            std::fs::create_dir_all(log_dir)?;
        }
        let start_offset = format!("{:020}", base_offset);
        let log_file_path = format!("{}/{}", log_dir, start_offset);
        let index_file_path = format!("{}/{}", log_dir, start_offset);
        let log_file = MutexFile::new(&log_file_path, "log")?;
        let index_file = MutexFile::new(&index_file_path, "index")?;
        let mmap_index = MmapIndex::new(&index_file.lock())?;

        //let offset: u64 = Self::recover_message_offset(&log_file, &index_file)?;
        // 如果指定了 offset，则直接使用；否则调用 recover_message_offset 恢复 offset
        let offset = match offset {
            Some(offset) => offset,
            None => Self::recover_message_offset(&log_file, &index_file)?,
        };

        Ok(Self {
            log_file,
            index_file,
            mmap_index,
            base_offset,
            offset,
            max_segment_size,
        })
    }

    //创建一个新的段，并替换当前段
    fn rotate_segment(&mut self) -> io::Result<()> {
        // 确保当前段的数据已经写入磁盘
        self.index_file.lock().flush()?;
        self.log_file.lock().flush()?;
        let new_segment = Self::with_offset(
            "logs",
            self.offset,
            self.max_segment_size,
            Some(self.offset),
        )?;
        *self = new_segment;
        Ok(())
    }

    pub fn append_message(&mut self, message: &[u8]) -> io::Result<u64> {
        if self.log_file.lock().metadata()?.len() as usize >= self.max_segment_size {
            self.rotate_segment()?;
        }

        let mut log_file = self.log_file.lock();
        let mut index_file = self.index_file.lock();

        let offset_bytes = self.offset.to_be_bytes();
        let length_bytes = (message.len() as u32).to_be_bytes();
        let log_pos = log_file.metadata()?.len();
        let log_pos_bytes = log_pos.to_be_bytes();

        log_file.write_all(&offset_bytes)?;
        log_file.write_all(&length_bytes)?;
        log_file.write_all(message)?;

        //log_file.flush()?; // ✅ 这里不立即 flush，避免频繁写入影响性能,

        if self.offset % 10 == 0 {
            index_file.write_all(&offset_bytes)?;
            index_file.write_all(&log_pos_bytes)?;
            //index_file.flush()?; // ✅ 这里不立即 flush，避免频繁写入影响性能 不立即flush,好像mmap获取不到信息？
        }

        let offset = self.offset; //相对偏移量（8 字节，相对于基准偏移量）
        self.offset += 1;
        Ok(offset)
    }

    // * 恢复消息偏移量
    fn recover_message_offset(log_file: &MutexFile, index_file: &MutexFile) -> io::Result<u64> {
        let mut log_file = log_file.lock(); // 解锁获取 File
        let mut index_file = index_file.lock();

        let index_size = index_file.metadata()?.len();
        if index_size < INDEX_ENTRY_SIZE as u64 {
            return Ok(0);
        }
        let mut last_offset = 0;
        let mut last_pos = 0;

        if index_size >= INDEX_ENTRY_SIZE as u64 {
            let mut buffer = [0u8; POS_SIZE];
            let last_offset_pos = index_size - INDEX_ENTRY_SIZE as u64 + OFFSET_SIZE as u64; // 8 字节 offset + 8 字节长度
            index_file.seek(SeekFrom::Start(last_offset_pos))?;
            index_file.read_exact(&mut buffer)?;
            //last_offset = u64::from_be_bytes(buffer[0..OFFSET_SIZE].try_into().unwrap());
            last_pos = u64::from_be_bytes(buffer); //定位到index的最大pos
        }

        log_file.seek(SeekFrom::Start(last_pos))?;
        let mut buffer = [0u8; MSG_HEADER_SIZE]; // 8 字节消息号 + 4 字节长度
        while log_file.read_exact(&mut buffer).is_ok() {
            last_offset = u64::from_be_bytes(buffer[0..OFFSET_SIZE].try_into().unwrap());
            let length =
                u32::from_be_bytes(buffer[OFFSET_SIZE..MSG_HEADER_SIZE].try_into().unwrap());
            log_file.seek(SeekFrom::Current(length as i64))?;
        }
        Ok(last_offset + 1 as u64)
    }

    /// 读取指定 offset 的消息    
    pub fn read_message(&mut self, offset: u64) -> io::Result<Option<Vec<u8>>> {
         let pos =  match self.mmap_index.find_position(offset){
            Some(pos) => pos,
            None => 0, //mmap不回记录所有的消息坐标。可能会返回None,属于合理情况
        };
        let mut log_file = self.log_file.lock();
            if pos >= log_file.metadata()?.len() {
                return Ok(None);
            }
            // **遍历日志文件，找到目标 offset**
            log_file.seek(SeekFrom::Start(pos))?;
            let mut buffer = [0u8; MSG_HEADER_SIZE];
            while log_file.read_exact(&mut buffer).is_ok() {
                let msg_offset = u64::from_be_bytes(buffer[0..OFFSET_SIZE].try_into().unwrap());
                let length =   u32::from_be_bytes(buffer[OFFSET_SIZE..MSG_HEADER_SIZE].try_into().unwrap()) as usize;
                if msg_offset == offset {
                    let mut message = vec![0u8; length];
                    log_file.read_exact(&mut message)?;
                    return Ok(Some(message));
                }
                log_file.seek(SeekFrom::Current(length as i64))?;
            }
        Ok(None) // 没有找到
    }
}
