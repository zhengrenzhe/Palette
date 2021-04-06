use crossbeam::queue::ArrayQueue;
use num_cpus::get_physical;
use std::cmp::max;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use crate::core::file::FileLoad;
use crate::core::pre_process::ConfigResult;
use crate::utils::file::ReadedFile;
use crate::utils::log;

pub struct Scheduler {
    _config: Arc<ConfigResult>,
    cpu_cores: usize,
    file_load: Arc<Mutex<FileLoad>>,
    file_loaded_queue: Arc<RwLock<ArrayQueue<ReadedFile>>>,
}

impl Scheduler {
    pub fn new(config: Arc<ConfigResult>) -> Scheduler {
        log::info("create scheduler to manage jobs");

        Scheduler {
            _config: config.clone(),
            cpu_cores: get_physical(),
            file_load: Arc::new(Mutex::new(FileLoad::new(config.clone()))),
            file_loaded_queue: Arc::new(RwLock::new(ArrayQueue::new(config.images.len()))),
        }
    }

    pub fn start(&self) {
        log::info(&format!("cpu has {} physical cores", self.cpu_cores));

        // 计算线程数量为：cpu物理总核数 - 文件读取线程 - 图片解码线程
        let calculate_cores = max(self.cpu_cores - 2, 1);

        log::info(&format!("threads allocation: [file load & result output thread] 1, [image decode thread] 1, [calculate thread] {}", calculate_cores));

        let file_load = self.file_load.clone();
        let file_loaded_queue = self.file_loaded_queue.clone();
        let file_load_handler =
            thread::spawn(move || file_load.lock().unwrap().start(file_loaded_queue));

        file_load_handler.join().unwrap();
    }
}
