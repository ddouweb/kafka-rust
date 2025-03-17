use memmap2::Mmap;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};

pub struct MmapLog {
    mmap: Mmap,
}

impl MmapLog {
    pub fn new(file: &File) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };
        Ok(Self { mmap })
    }

    pub fn read_message(&self, offset: usize) -> Option<&[u8]> {
        if offset + 12 > self.mmap.len() {
            return None;
        }
        let length = u32::from_be_bytes(self.mmap[offset + 8..offset + 12].try_into().unwrap()) as usize;
        Some(&self.mmap[offset + 12..offset + 12 + length])
    }
}
