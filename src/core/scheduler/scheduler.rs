use crossbeam::channel::unbounded;
use crossbeam::queue::ArrayQueue;
use num_cpus::get_physical;
use std::cmp::max;
use std::sync::{Arc, RwLock};
use std::thread;

use crate::core::file::FileLoad;
use crate::core::image_parse::ImageParse;
use crate::core::pre_process::ConfigResult;
use crate::core::scheduler::SchedulerChannelMsg;
use crate::utils::file::ReadedFile;
use crate::utils::log;

pub struct Scheduler {
    config: Arc<ConfigResult>,
    cpu_cores: usize,
    file_loaded_queue: Arc<RwLock<ArrayQueue<ReadedFile>>>,
}

impl Scheduler {
    pub fn new(config: Arc<ConfigResult>) -> Scheduler {
        log::info("create scheduler to manage jobs");

        Scheduler {
            config: config.clone(),
            cpu_cores: get_physical(),
            file_loaded_queue: Arc::new(RwLock::new(ArrayQueue::new(config.images.len()))),
        }
    }

    pub fn start(&self) {
        log::info(&format!("cpu has {} physical cores", self.cpu_cores));

        // 计算线程数量为：cpu物理总核数 - 文件读取线程 - 图片解码线程
        let calculate_cores = max(self.cpu_cores - 2, 1);

        log::info(&format!("threads allocation: [file load & result output thread] 1, [image decode thread] 1, [calculate thread] {}", calculate_cores));

        let (sender, receiver) = unbounded::<SchedulerChannelMsg>();

        // 创建file_load线程，顺序加载图片，加载完成后线程退出
        let file_load = FileLoad::new(self.config.clone(), self.file_loaded_queue.clone());
        thread::spawn(move || file_load.start());

        for _ in 0..calculate_cores {
            let image_parse = ImageParse::new(self.file_loaded_queue.clone());
            thread::spawn(move || image_parse.start());
        }

        for msg in receiver {
            match msg {
                SchedulerChannelMsg::StopScheduler => break,
                SchedulerChannelMsg::ImageLoaded => {}
            }
        }
    }
}
