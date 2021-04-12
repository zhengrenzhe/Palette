use std::sync::{Arc, RwLock};

use crate::core::image_decode::decode;
use crate::core::image_decode::ImageRaw;
use crate::utils::file::ReadedFile;
use crate::utils::log;
use crate::utils::queue::Queue;

pub struct ImageDecode {
    file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
    image_raw_queue: Arc<RwLock<Queue<ImageRaw>>>,
}

impl ImageDecode {
    pub fn new(
        file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
        image_raw_queue: Arc<RwLock<Queue<ImageRaw>>>,
    ) -> ImageDecode {
        ImageDecode {
            file_loaded_queue,
            image_raw_queue,
        }
    }

    pub fn start(&self) {
        log::info("create image parse thread");
        loop {
            if let Ok(queue) = self.file_loaded_queue.read() {
                if let Some(f) = queue.pop() {
                    match decode(f.buffer) {
                        Ok(decoded) => {
                            log::success(&format!("image:{} decode success", &f.path));

                            self.image_raw_queue
                                .write()
                                .unwrap()
                                .push(ImageRaw::new(decoded, f.path));
                        }
                        Err(err) => {
                            log::error(&format!("decode {} is error: {}", &f.path, err));
                        }
                    };
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
