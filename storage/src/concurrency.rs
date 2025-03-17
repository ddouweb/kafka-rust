use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};

pub struct MutexFile {
    pub file: Arc<Mutex<std::fs::File>>,
}

impl MutexFile {
    pub fn new(path: &str, ext: &str) -> std::io::Result<Self> {
        let file_path = format!("{}.{}", path, ext);
        //println!("file_path:{}",file_path.clone().to_string());
        let file = OpenOptions::new().create(true).append(true).read(true).open(file_path)?;
        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }

    pub fn lock(&self) -> std::sync::MutexGuard<'_, std::fs::File> {
        self.file.lock().unwrap()
    }
}
