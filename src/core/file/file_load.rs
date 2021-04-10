use std::sync::{Arc, RwLock};

use crate::core::pre_process::ConfigResult;
use crate::utils::file::{read, ReadedFile};
use crate::utils::log;
use crate::utils::queue::Queue;

pub struct FileLoad {
    config_result: Arc<ConfigResult>,
    file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
}

impl FileLoad {
    pub fn new(
        config_result: Arc<ConfigResult>,
        file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
    ) -> FileLoad {
        FileLoad {
            config_result,
            file_loaded_queue,
        }
    }

    pub fn start(&mut self) {
        let images = self.config_result.images.clone();
        log::info(&format!("start load {} images", images.len()));

        for img in images.iter() {
            if let Some(content) = read(img) {
                self.file_loaded_queue.write().unwrap().push(content)
            };
        }

        self.file_loaded_queue.write().unwrap().set_all_push_done();
        log::success("all images loaded, file load thread will exit");
    }
}
