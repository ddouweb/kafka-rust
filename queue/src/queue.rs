use storage::LogSegment;
use std::collections::VecDeque;
use std::io;

use storage::LOG_FILE_SUFFIX;
pub struct LogQueue {
    segments: VecDeque<LogSegment>, // 存储多个日志段
    log_dir: String,                // 日志存储路径
    max_segment_size: usize,        // 每个日志段的最大大小
}

impl LogQueue {
    pub fn new(log_dir: &str, max_segment_size: usize) -> io::Result<Self> {
        let mut queue = Self {
            segments: VecDeque::new(),
            log_dir: log_dir.to_string(),
            max_segment_size,
        };
        queue.load_segments()?;
        Ok(queue)
    }

    /// 加载已有的日志段
    fn load_segments(&mut self) -> io::Result<()> {
        // 获取目录下所有日志段
        let mut segment_offsets: Vec<u64> = std::fs::read_dir(&self.log_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name();
                let name = file_name.to_str()?;
                if name.ends_with(LOG_FILE_SUFFIX) {
                    name.trim_end_matches(LOG_FILE_SUFFIX).parse::<u64>().ok()
                } else {
                    None
                }
            })
            .collect();
        
        segment_offsets.sort();

        // 加载所有日志段
        for offset in segment_offsets {
            let segment = LogSegment::new(&self.log_dir, offset, self.max_segment_size)?;
            self.segments.push_back(segment);
        }

        // 如果没有找到任何日志段，创建一个新的
        if self.segments.is_empty() {
            let new_segment = LogSegment::new(&self.log_dir, 0, self.max_segment_size)?;
            self.segments.push_back(new_segment);
        }

        Ok(())
    }

    /// 追加消息，自动选择合适的日志段
    pub fn append_message(&mut self, message: &[u8]) -> io::Result<u64> {
        if let Some(segment) = self.segments.back_mut() {
            match segment.append_message(message) {
                Ok(offset) => return Ok(offset), // 1. 写入成功，返回 offset
                Err(e) if e.kind() == io::ErrorKind::Other && e.to_string() == "Segment full" => {
                    // 2. 发现段已满，创建新段
                    println!("Segment full, rotating to new segment...");
                }
                Err(e) => return Err(e), // 其他 IO 错误，直接返回
            }
        }

        // 如果当前日志段已满，则创建新段
        let mut new_segment = LogSegment::new(&self.log_dir, self.get_next_base_offset(), self.max_segment_size)?;
        let offset = new_segment.append_message(message)?;
        self.segments.push_back(new_segment);
        Ok(offset)
    }

    /// 读取指定 offset 的消息
    pub fn read_message(&mut self, offset: u64) -> io::Result<Option<Vec<u8>>> {
        for segment in self.segments.iter_mut() {
            if let Some(message) = segment.read_message(offset)? {
                return Ok(Some(message));
            }
        }
        Ok(None)
    }

    /// 获取下一个日志段的起始 offset
    fn get_next_base_offset(&self) -> u64 {
        self.segments.back().map(|s| s.get_next_offset()).unwrap_or(0)
    }
}
