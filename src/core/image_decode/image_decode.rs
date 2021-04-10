use std::sync::{Arc, RwLock};

use crate::utils::file::ReadedFile;
use crate::utils::log;
use crate::utils::queue::Queue;

pub struct ImageDecode {
    file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
}

impl ImageDecode {
    pub fn new(file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>) -> ImageDecode {
        ImageDecode { file_loaded_queue }
    }

    pub fn start(&self) {
        log::info("create image parse thread");
        loop {
            if let Ok(queue) = self.file_loaded_queue.read() {
                if let Some(f) = queue.pop() {
                    log::info(&format!("got file {}", f.path));
                }

                if queue.is_pushed() && queue.is_empty() && queue.is_all_push_done() {
                    log::success("no image to decode, this thread will exit");
                    break;
                }
            } else {
                log::error(&"got file_loaded_queue read lock error".to_string());
            }
        }
    }
}
