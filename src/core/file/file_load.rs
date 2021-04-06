use crossbeam::queue::ArrayQueue;
use std::sync::{Arc, RwLock};

use crate::core::pre_process::ConfigResult;
use crate::utils::file::{read, ReadedFile};
use crate::utils::log;

pub struct FileLoad {
    config_result: Arc<ConfigResult>,
}

impl FileLoad {
    pub fn new(config_result: Arc<ConfigResult>) -> FileLoad {
        FileLoad { config_result }
    }

    pub fn start(&self, file_loaded_queue: Arc<RwLock<ArrayQueue<ReadedFile>>>) {
        let images = self.config_result.images.clone();
        log::info(&format!("start load {} images", images.len()));

        match file_loaded_queue.read() {
            Ok(queue) => {
                for img in images.iter() {
                    match read(img) {
                        None => {
                            log::warning(&format!("read file {} error, this file will ignore", img))
                        }
                        Some(image_content) => match queue.push(image_content) {
                            Ok(_) => log::info(&format!("{} in queue success", img)),
                            Err(_) => log::error(&format!("{} in queue error", img)),
                        },
                    }
                }
            }
            Err(err) => log::error(&format!("get file_loaded_queue lock error: {}", err)),
        }
    }
}
