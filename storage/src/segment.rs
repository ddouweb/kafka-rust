use super::{INDEX_FILE_SUFFIX, LOG_FILE_SUFFIX, MSG_HEADER_SIZE, OFFSET_SIZE};
use crate::concurrency::MutexFile;
use crate::io_result::IoResult;
use crate::mmap::MmapIndex;
use std::io::{self, Read, Seek, SeekFrom, Write};
pub struct LogSegment {
    log_file: MutexFile,     // 存储实际消息数据
    index_file: MutexFile,   // 存储索引 //todo 待优化：是否预分配空间？
    mmap_index: MmapIndex,   // 存储索引,使用mmap
    base_offset: u64,        // 当前段的起始 offset
    offset: u64,             // 下一个消息的 offset
    max_segment_size: usize, // 单个段的最大大小
}

impl LogSegment {
    pub fn new(log_dir: &str, base_offset: u64, max_segment_size: usize) -> io::Result<Self> {
        let log_dir = log_dir.trim_end_matches('/');

        if !std::path::Path::new(log_dir).exists() {
            std::fs::create_dir_all(log_dir)?;
        }

        let start_offset = format!("{:020}", base_offset);
        let log_file_path = format!("{}/{}{}", log_dir, start_offset, LOG_FILE_SUFFIX);
        let index_file_path = format!("{}/{}{}", log_dir, start_offset, INDEX_FILE_SUFFIX);

        let log_file = MutexFile::new(&log_file_path)?;
        let index_file = MutexFile::new(&index_file_path)?;
        let mmap_index = MmapIndex::new(&index_file.lock())?;

        // 判断日志文件是否为空，如果为空，则使用 base_offset 否则从文件中恢复
        let offset = {
            let mut file = log_file.lock(); // 锁定文件进行操作
            if file.metadata()?.len() == 0 {
                base_offset // 文件为空时，偏移量从 base_offset 开始
            } else {
                // 恢复消息的偏移量
                Self::recover_message_offset(&mut file, &mmap_index)?
            }
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

    pub fn append_message(&mut self, message: &[u8]) -> io::Result<IoResult> {
        let mut log_file = self.log_file.lock();
        let mut index_file = self.index_file.lock();
        let file_len = log_file.metadata()?.len();
        if file_len >= self.max_segment_size as u64 {
            log_file.flush()?;
            index_file.flush()?;
            self.mmap_index = MmapIndex::new(&index_file)?;
            return Ok(IoResult::SegmentFull);
        }

        let offset_bytes = self.offset.to_be_bytes();
        let log_pos_bytes = file_len.to_be_bytes();

        let mut buffer = Vec::with_capacity(MSG_HEADER_SIZE + message.len());
        buffer.extend_from_slice(&offset_bytes);
        buffer.extend_from_slice(&(message.len() as u32).to_be_bytes());
        buffer.extend_from_slice(message);

        log_file.write_all(&buffer)?;

        if self.offset % 100 == 0 {
            let index_entry = [&offset_bytes[..], &log_pos_bytes[..]].concat();
            log_file.flush()?;
            index_file.write_all(&index_entry)?;
            index_file.flush()?;
            self.mmap_index = MmapIndex::new(&index_file)?; // 重新映射 mmap
        }

        let offset = self.offset; //相对偏移量（8 字节，相对于基准偏移量）
        self.offset += 1;
        Ok(IoResult::Success(offset))
    }

    // * 恢复消息偏移量
    fn recover_message_offset(log_file: &mut std::sync::MutexGuard<'_, std::fs::File>, mmap_index: &MmapIndex) -> io::Result<u64> {

        let (mut last_offset, last_pos) = mmap_index.last_entry().unwrap_or((0, 0));

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
        let pos = match self.mmap_index.find_position(offset) {
            Some(pos) => pos,
            None => {
                if self.offset > 1000 {
                    self.mmap_index
                        .find_position(self.offset - 1000)
                        .unwrap_or(0) //如果 mmap_index 里没有找到 offset，从 0 位置开始查找可能会导致全文件扫描，影响性能。
                } else {
                    0
                }
            }
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

    // 清理旧的段
    // pub fn cleanup_old_segments(&mut self, max_size: u64, max_age_secs: u64) -> io::Result<()> {
    //     let log_dir = std::fs::read_dir("logs")?;
    //     let mut total_size = 0;
    //     let now = SystemTime::now();

    //     let mut segments: Vec<(u64, PathBuf, Metadata)> = log_dir.filter_map(|entry| {
    //         let entry = entry.ok()?;
    //         let metadata = entry.metadata().ok()?;
    //         let name = entry.file_name().into_string().ok()?;
    //         let offset = name.parse::<u64>().ok()?;
    //         Some((offset, entry.path(), metadata))
    //     }).collect();

    //     segments.sort_by_key(|s| s.0); // 按 offset 递增排序

    //     for (offset, path, metadata) in segments {
    //         total_size += metadata.len();
    //         if total_size > max_size || metadata.modified()?.elapsed().unwrap().as_secs() > max_age_secs {
    //             std::fs::remove_file(path)?;
    //             println!("Deleted old segment: {:?}", path);
    //         }
    //     }

    //     Ok(())
    // }

    //创建一个新的段，并替换当前段
    // fn rotate_segment(&mut self) -> io::Result<()> {
    //     // 确保当前段的数据已经写入磁盘
    //     self.index_file.lock().flush()?;
    //     self.log_file.lock().flush()?;
    //     let new_segment = Self::with_offset(
    //         "logs",
    //         self.offset,
    //         self.max_segment_size,
    //         Some(self.offset),
    //     )?;
    //     *self = new_segment;
    //     Ok(())
    // }

    pub fn get_next_offset(&self) -> u64 {
        self.offset
    }

    //返回当前段大小
    pub fn get_size(&self) -> usize {
        self.log_file.lock().metadata().unwrap().len() as usize
    }

    // //返回全局唯一的offset
    // pub fn get_next_offset() -> u64 {
    //     super::GLOBAL_OFFSET.fetch_add(1, Ordering::SeqCst)
    // }
}
