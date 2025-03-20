pub mod segment;
pub mod mmap;
pub mod retention;
pub mod concurrency;

// 对外暴露核心 API
pub use segment::LogSegment;
pub use retention::clean_old_segments;
use std::sync::atomic::AtomicU64;

const MSG_LEN_SIZE: usize = 4; // 消息长度占 4 字节
const OFFSET_SIZE: usize = 8; // 相对偏移量 占 8 字节
const POS_SIZE: usize = 8; // 物理偏移量 占 8 字节
const INDEX_ENTRY_SIZE: usize = OFFSET_SIZE + POS_SIZE; // 每个索引条目 8+8=16 字节（相对偏移量 + 物理偏移量）
const MSG_HEADER_SIZE: usize = OFFSET_SIZE + MSG_LEN_SIZE; // 日志条目头部 8+4=12 字节
static GLOBAL_OFFSET: AtomicU64 = AtomicU64::new(0);