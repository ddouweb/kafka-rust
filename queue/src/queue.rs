use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::io;
use storage::IoResult;
use storage::LogSegment;
use storage::LOG_FILE_SUFFIX;

#[derive(Debug)]
pub struct LogQueue {
    segments: VecDeque<LogSegment>, // 存储多个日志段  //后续考虑优化，是否会存在并发访问的情况？
    log_dir: String,                // 日志存储路径
    max_segment_size: usize,        // 每个日志段的最大大小
    //max_queue_size: usize,          // 队列的最大大小
    active_write_segment_index: usize,   // 当前活跃的写入 segment
    active_read_segment_index: usize,    // 当前活跃的读取 segment
    segment_index: BTreeMap<u64, usize>, // 存储每个base_offset -> segment_index
}

impl LogQueue {
    pub fn new(
        log_dir: &str,
        max_segment_size: usize, /*, max_queue_size:usize*/
    ) -> io::Result<Self> {
        let mut queue = Self {
            segments: VecDeque::new(),
            log_dir: log_dir.to_string(),
            max_segment_size,
            // /max_queue_size,
            active_write_segment_index: 0,
            active_read_segment_index: 0,
            segment_index: BTreeMap::new(),
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
        for (index, offset) in segment_offsets.iter().enumerate() {
            let segment = LogSegment::new(&self.log_dir, *offset, self.max_segment_size)?;
            self.segments.push_back(segment);
            self.segment_index.insert(*offset, index);
        }

        // 如果没有找到任何日志段，创建一个新的
        if self.segments.is_empty() {
            let new_offset = 0;
            let new_segment = LogSegment::new(&self.log_dir, new_offset, self.max_segment_size)?;
            self.segments.push_back(new_segment);
            self.segment_index.insert(0, 0);
        }

        // 写入 segment 指向最后一个
        self.active_write_segment_index = self.segments.len() - 1;
        // 读取 segment 默认从第一个开始
        self.active_read_segment_index = 0;

        Ok(())
    }

    /// 追加消息，自动选择合适的日志段
    pub fn append_message(&mut self, message: &[u8]) -> io::Result<u64> {
        if let Some(segment) = self.segments.get_mut(self.active_write_segment_index) {
            match segment.append_message(message) {
                Ok(IoResult::Success(offset)) => return Ok(offset),
                Ok(IoResult::SegmentFull) => { /*当前日志段已满，不做任何处理，后续处理段和消息写入*/
                }
                Err(e) => return Err(e),
            }
        }
        // 如果当前日志段已满，则创建新段
        let new_offset = self.get_next_base_offset();
        let mut new_segment = LogSegment::new(&self.log_dir, new_offset, self.max_segment_size)?;
        let result = new_segment.append_message(message)?;
        self.segments.push_back(new_segment);
        let index = self.segments.len() - 1;
        self.active_write_segment_index = index;
        self.segment_index.insert(new_offset, index);
        Ok(match result {
            IoResult::Success(offset) => offset,
            //抛出IO异常
            IoResult::SegmentFull => panic!("SegmentFull should not happen here"),
        })
    }

    /// 读取指定 offset 的消息 ，首次读取使用索引快速查找log segment,后续则通过active_read_segment_index查找读取
    pub fn read_message(&mut self, offset: u64) -> io::Result<Option<Vec<u8>>> {
        if self.active_read_segment_index == 0 {
            // 1. **使用索引快速查找 segment**
            if let Some((_, &index)) = self.segment_index.range(..=offset).next_back() {
                if let Some(segment) = self.segments.get_mut(index) {
                    self.active_read_segment_index = index;
                    return segment.read_message(offset);
                }
            }
        }
        // **从当前的 active_read_segment 开始读**
        if let Some(segment) = self.segments.get_mut(self.active_read_segment_index) {
            if let Some(message) = segment.read_message(offset)? {
                return Ok(Some(message));
            }
        }
        // **如果 offset 超出了当前的 active_read_segment，向前查找**
        for (index, segment) in self
            .segments
            .iter_mut()
            .enumerate()
            // index是从0开始的，所以skip要加1。避免重复读不到消息浪费IO
            .skip(self.active_read_segment_index + 1)
        {
            if let Some(message) = segment.read_message(offset)? {
                self.active_read_segment_index = index; // 更新 active_read_segment
                return Ok(Some(message));
            }
        }
        Ok(None)
    }

    /// 获取下一个日志段的起始 offset
    fn get_next_base_offset(&self) -> u64 {
        self.segments
            .back()
            .map(|s| s.get_next_offset())
            .unwrap_or(0)
    }
}
