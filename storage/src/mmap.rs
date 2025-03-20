use memmap2::Mmap;
use std::fs::File;
use std::io::{self};
use super::{OFFSET_SIZE,INDEX_ENTRY_SIZE,POS_SIZE};
pub struct MmapIndex {
    mmap: Mmap,
}

impl MmapIndex {
    /// **加载索引文件**
    pub fn new(index_file: &File) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(index_file)? };
        Ok(Self { mmap })
    }

    /// **查找目标 offset 对应的日志文件位置**
    pub fn find_offset(&self, target_offset: u64) -> Option<u64> {

        let index_size = self.mmap.len();
        if index_size == 0 {
            return None;
        }

        let mut closest_offset = 0;
        let mut closest_pos = 0;

        let mut index = 0;
        while index + INDEX_ENTRY_SIZE <= index_size {
            let stored_offset =
                u64::from_be_bytes(self.mmap[index..index + OFFSET_SIZE].try_into().unwrap());
            let log_pos = u64::from_be_bytes(self.mmap[index + OFFSET_SIZE..index + INDEX_ENTRY_SIZE].try_into().unwrap());

            if stored_offset > target_offset {
                break;
            }

            closest_offset = stored_offset;
            closest_pos = log_pos;
            index += INDEX_ENTRY_SIZE;
        }

        if closest_offset > target_offset {
            None
        } else {
            Some(closest_pos)
        }
    }

    /// **二分查找最近的 offset**
    pub fn find_offset_half(&self, target_offset: u64) -> Option<u64> {
        let num_entries = self.mmap.len() / INDEX_ENTRY_SIZE;

        let mut low = 0;
        let mut high = num_entries;

        while low < high {
            let mid = (low + high) / 2;
            let entry_start = mid * INDEX_ENTRY_SIZE;
            let stored_offset = u64::from_be_bytes(self.mmap[entry_start..entry_start + OFFSET_SIZE].try_into().unwrap());

            if stored_offset < target_offset {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        if low < num_entries {
            let entry_start = low * INDEX_ENTRY_SIZE;
            let position = u64::from_be_bytes(self.mmap[entry_start + POS_SIZE..entry_start + INDEX_ENTRY_SIZE].try_into().unwrap());
            Some(position)
        } else {
            None
        }
    }
}
