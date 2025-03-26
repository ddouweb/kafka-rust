use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use queue::LogQueue;
use protocol::{ClientRequest, FetchRequest, JoinGroupRequest, MetadataRequest, OffsetFetchRequest, ProduceRequest, SyncGroupRequest};

pub struct Broker {
    topics: Arc<Mutex<HashMap<String, Vec<LogQueue>>>>, // 维护 Topic -> Partitions
    offsets: Arc<Mutex<HashMap<String, HashMap<String, u32>>>>, // 存储 Consumer Group 的 offset
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            topics: Arc::new(Mutex::new(HashMap::new())),
            offsets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 创建 Topic
    pub fn create_topic(&self, topic: &str, partitions: usize) {
        // let mut topics = self.topics.lock().unwrap();
        // if !topics.contains_key(topic) {
        //     let queues = (0..partitions).map(|_| LogQueue::new()).collect();
        //     topics.insert(topic.to_string(), queues);
        // }
        todo!()
    }

    // 发送消息
    pub fn send_message(&self, topic: &str, message: Vec<u8>) -> std::io::Result<u64> {
        // let mut topics = self.topics.lock().unwrap();
        // if let Some(partitions) = topics.get_mut(topic) {
        //     let partition_index = (message.len() % partitions.len()) as usize;
        //     let queue = &mut partitions[partition_index];
        //     return queue.append_message(message);
        // }
        Ok(1)
    }

    // 读取消息
    pub fn fetch_message(&self, topic: &str, partition: usize, offset: u32) -> Option<Vec<u8>> {
        // let topics = self.topics.lock().unwrap();
        // topics.get(topic)
        //     .and_then(|partitions| partitions.get(partition))
        //     .and_then(|queue| queue.read_message(offset))
        todo!("fetch_message")
    }

    // 提交 offset
    pub fn commit_offset(&self, group: &str, topic: &str, partition: usize, offset: u32) {
        let mut offsets = self.offsets.lock().unwrap();
        let group_offsets = offsets.entry(group.to_string()).or_insert(HashMap::new());
        group_offsets.insert(format!("{topic}-{partition}"), offset);
    }

    // 获取 offset
    pub fn get_offset(&self, group: &str, topic: &str, partition: usize) -> Option<u32> {
        let offsets = self.offsets.lock().unwrap();
        offsets.get(group)
            .and_then(|group_offsets| group_offsets.get(&format!("{topic}-{partition}")))
            .copied()
    }

    pub fn handle_request(&self, request: ClientRequest) -> Result<(), String> {
        // match request {
        //     ClientRequest::Produce(req) => {
        //         self.send_message(&req.topic, req.partition, req.message);
        //     }
        //     ClientRequest::Fetch(req) => {
        //         self.fetch_message(&req.topic, req.partition, req.offset);
        //     }
        //     ClientRequest::Metadata(req) => {
        //         self.get_metadata(req.topic);
        //     }
        //     ClientRequest::OffsetFetch(req) => {
        //         self.get_offset(&req.group_id, &req.topic, req.partition);
        //     }
        //     ClientRequest::JoinGroup(req) => {
        //         self.join_group(&req.group_id, &req.consumer_id);
        //     }
        //     ClientRequest::SyncGroup(req) => {
        //         self.sync_group(&req.group_id, &req.consumer_id, req.assignments);
        //     }
        // }
        Ok(())
    }
    
}
