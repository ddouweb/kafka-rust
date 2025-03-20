//use memmap2::Mmap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const MSG_LEN_SIZE: usize = 4; // 消息长度占 4 字节
const MSG_ADD_OFFSET: usize = 1;
const OFFSET_SIZE: usize = 8; // 相对偏移量 占 8 字节
const POS_SIZE: usize = 8; // 物理偏移量 占 8 字节
const INDEX_ENTRY_SIZE: usize = OFFSET_SIZE + POS_SIZE; // 每个索引条目 8+8=16 字节（相对偏移量 + 物理偏移量）
const MSG_HEADER_SIZE: usize = OFFSET_SIZE + MSG_LEN_SIZE; // 日志条目头部 8+4=12 字节

/// 单个日志段
pub struct LogSegment {
    log_file: File,   // 存储实际消息数据
    index_file: File, // 存储索引
    //base_offset: u64,        // 当前段的起始 offset
    offset: u64,             // 下一个消息的 offset
    max_segment_size: usize, // 单个段的最大大小
    log_path: String,        // 追加日志文件路径
    index_path: String,      // 索引文件路径
}

impl LogSegment {
    /// 创建或打开一个日志段
    pub fn new(path: &str, max_segment_size: usize) -> io::Result<Self> {
        let log_filename = format!("{}.log", path);
        let index_filename = format!("{}.index", path);

        let log_path = Path::new(&log_filename);
        let index_path = Path::new(&index_filename);
        // let log_path = log_filename.clone();
        // let index_path = index_filename.clone();

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(log_path)?;

        let index_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(index_path)?;

        // todolet mmap_index_file = unsafe { Mmap::map(&index_file)? };
        // todo let index_data: &[u8] = &mmap_index_file[..];

        // todolet next_offset = base_offset + LogSegment::load_last_offset(&index_file)?;

        let offset = LogSegment::recover_message_offset( &log_file,&index_file)?;

        Ok(Self {
            log_file,
            index_file,
            //base_offset,
            offset,
            max_segment_size,
            log_path: log_filename,
            index_path: index_filename,
        })
    }

    /// 追加消息到日志段
    pub fn append_message(&mut self, message: &[u8]) -> io::Result<u64> {
        let offset = self.offset; //相对偏移量（8 字节，相对于基准偏移量）
        let offset_bytes = offset.to_be_bytes();

        let length_bytes = (message.len() as u32).to_be_bytes();
        let log_pos = self.log_file.metadata()?.len(); // 物理位置
        let log_pos_bytes = (log_pos).to_be_bytes(); //物理位置（8 字节，消息在 .log 文件中的字节偏移）

        // 检查是否超出最大段大小
        if log_pos as usize >= self.max_segment_size {
            return Err(io::Error::new(io::ErrorKind::Other, "Segment full"));
        }

        // 按照 Kafka 格式写入日志文件： offset + length + message
        self.log_file.write_all(&offset_bytes)?; //8
        self.log_file.write_all(&length_bytes)?; //4
        self.log_file.write_all(message)?; //
        self.log_file.flush()?; // ✅ 这里不立即 flush，避免频繁写入影响性能

        self.index_file.write_all(&offset_bytes)?; //8
        self.index_file.write_all(&log_pos_bytes)?; //8
        self.index_file.flush()?; // ✅ 这里不立即 flush，避免频繁写入影响性能

        self.offset += MSG_ADD_OFFSET as u64; // Kafka 的 消息号 是递增的
        Ok(offset) //返回当前的消息号s
    }

    /// 读取指定 offset 的消息    
    pub fn read_message(&mut self, offset: u64) -> io::Result<Option<Vec<u8>>> {
        let mut index_file = self.index_file.lock();
        let index_size = index_file.metadata()?.len();

        if index_size == 0 {
            return Ok(None);
        }

        // **遍历索引文件，找到最近的偏移量**
        let mut closest_offset = 0;
        let mut closest_pos = 0;
        let mut index_buffer = [0u8; INDEX_ENTRY_SIZE];

        index_file.seek(SeekFrom::Start(0))?;
        while index_file.read_exact(&mut index_buffer).is_ok() {
            let stored_offset =
                u64::from_be_bytes(index_buffer[0..OFFSET_SIZE].try_into().unwrap());
            let log_pos = u64::from_be_bytes(
                index_buffer[OFFSET_SIZE..INDEX_ENTRY_SIZE]
                    .try_into()
                    .unwrap(),
            );

            if stored_offset > offset {
                break; // 取最近的小于等于 offset 的条目
            }

            closest_offset = stored_offset;
            closest_pos = log_pos;
        }

        if closest_offset > offset {
            return Ok(None);
        }
        let mut log_file = self.log_file.lock();
        // **遍历日志文件，找到目标 offset**
        log_file.seek(SeekFrom::Start(closest_pos))?;
        let mut buffer = [0u8; MSG_HEADER_SIZE];

        while log_file.read_exact(&mut buffer).is_ok() {
            let msg_offset = u64::from_be_bytes(buffer[0..OFFSET_SIZE].try_into().unwrap());
            let length =
                u32::from_be_bytes(buffer[OFFSET_SIZE..MSG_HEADER_SIZE].try_into().unwrap())
                    as usize;

            if msg_offset == offset {
                let mut message = vec![0u8; length];
                log_file.read_exact(&mut message)?;
                return Ok(Some(message));
            }

            log_file.seek(SeekFrom::Current(length as i64))?;
        }

        Ok(None) // 没有找到
    }
    

    fn recover_message_offset(mut log_file: &File,mut index_file: &File) -> io::Result<u64> {
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
            let length = u32::from_be_bytes(buffer[OFFSET_SIZE..MSG_HEADER_SIZE].try_into().unwrap());
            log_file.seek(SeekFrom::Current(length as i64))?;
        }
        Ok(last_offset+MSG_ADD_OFFSET as u64)
    }

    pub fn clean_files(&mut self) {
        // // 先关闭文件
        // let _ =  drop(&self.log_file);
        // let _ =  drop(&self.index_file);

        // 删除文件
        if let Err(e) = std::fs::remove_file(&self.log_path) {
            eprintln!("Failed to remove log file {}: {}", self.log_path, e);
        }
        if let Err(e) = std::fs::remove_file(&self.index_path) {
            eprintln!("Failed to remove index file {}: {}", self.index_path, e);
        }
    }
}