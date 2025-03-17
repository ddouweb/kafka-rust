pub mod segment;
pub mod mmap;
pub mod retention;
pub mod concurrency;

// 对外暴露核心 API
pub use segment::LogSegment;
pub use retention::clean_old_segments;