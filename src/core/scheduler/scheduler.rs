use num_cpus;
use std::cmp::max;

use crate::core::pre_process::ConfigResult;
use crate::utils::log;

pub struct Scheduler {
    config: ConfigResult,
    cpu_cores: usize,
}

impl Scheduler {
    pub fn new(config: ConfigResult) -> Scheduler {
        log::info("create scheduler to manage jobs");

        Scheduler {
            config,
            cpu_cores: num_cpus::get_physical(),
        }
    }

    pub fn start(&self) {
        log::info(&format!("cpu has {} physical cores", self.cpu_cores));

        /// 计算线程数量为：cpu物理总核数 - 文件读取线程 - 图片解码线程
        let calculate_cores = max(self.cpu_cores - 2, 1);

        log::info(&format!("threads allocation: [file load & result output thread] 1, [image decode thread] 1, [calculate thread] {}", calculate_cores));
    }
}
