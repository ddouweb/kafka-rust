use std::fs;
use std::time::{Duration, SystemTime};
use std::thread;

const RETENTION_TIME: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 7 天
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB

pub fn clean_old_segments(log_dir: String) {
    thread::spawn(move || {
        loop {
            if let Ok(entries) = fs::read_dir(&log_dir) {
                let mut segments = entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "log"))
                    .collect::<Vec<_>>();

                segments.sort_by_key(|entry| entry.metadata().unwrap().created().unwrap());

                let mut total_size = 0;
                let now = SystemTime::now();

                for segment in &segments {
                    let metadata = segment.metadata().unwrap();
                    total_size += metadata.len();

                    if total_size > MAX_LOG_SIZE || now.duration_since(metadata.created().unwrap()).unwrap() > RETENTION_TIME {
                        fs::remove_file(segment.path()).unwrap();
                        let index_path = segment.path().with_extension("index");
                        fs::remove_file(index_path).unwrap();
                    }
                }
            }

            thread::sleep(Duration::from_secs(60 * 60));
        }
    });
}
