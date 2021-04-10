use num_cpus::get_physical;
use std::cmp::max;
use std::sync::{Arc, RwLock};

use crate::core::file::FileLoad;
use crate::core::image_decode::{ImageDecode, ImageRaw};
use crate::core::pre_process::ConfigResult;
use crate::utils::file::ReadedFile;
use crate::utils::log;
use crate::utils::queue::Queue;
use crate::utils::thread::new_thread;

pub struct Scheduler {
    config: Arc<ConfigResult>,
    cpu_cores: usize,
    file_loaded_queue: Arc<RwLock<Queue<ReadedFile>>>,
    image_raw_queue: Arc<RwLock<Queue<ImageRaw>>>,
}

impl Scheduler {
    pub fn new(config: Arc<ConfigResult>) -> Scheduler {
        log::info("create scheduler to manage jobs");

        Scheduler {
            config: config.clone(),
            cpu_cores: get_physical(),
            file_loaded_queue: Arc::new(RwLock::new(Queue::new(
                config.images.len(),
                "file_loaded",
            ))),
            image_raw_queue: Arc::new(RwLock::new(Queue::new(config.images.len(), "image_row"))),
        }
    }

    pub fn start(&self) {
        log::info(&format!(
            "use {} mode",
            if self.config.gui { "gui" } else { "log" }
        ));
        log::info(&format!("cpu has {} physical cores", self.cpu_cores));

        let calculate_cores = max(self.cpu_cores - 2, 1);

        let mut handles = Vec::new();

        log::info(&format!("threads allocation: [file load & result output thread] 1, [image decode thread] 1, [calculate thread] {}", calculate_cores));

        // create file load thread, load all images from config, this thread will exit after all images loaded.
        let mut file_load = FileLoad::new(self.config.clone(), self.file_loaded_queue.clone());
        handles.push(new_thread("FileLoad").spawn(move || file_load.start()));

        // create image decode thread, decode all images from file_load queue
        let image_decode =
            ImageDecode::new(self.file_loaded_queue.clone(), self.image_raw_queue.clone());
        handles.push(new_thread("ImageDecode").spawn(move || image_decode.start()));

        for handle in handles {
            handle.unwrap().join().unwrap();
        }
    }
}
