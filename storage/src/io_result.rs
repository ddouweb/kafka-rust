/// 追加消息的结果
pub enum IoResult {
    Success(u64),  // 成功写入，返回 offset
    SegmentFull
}
