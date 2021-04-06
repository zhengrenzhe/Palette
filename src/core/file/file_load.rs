use std::sync::Arc;

use crate::core::pre_process::ConfigResult;
use crate::utils::log;

pub struct FileLoad {
    config_result: Arc<ConfigResult>,
}

impl FileLoad {
    pub fn new(config_result: Arc<ConfigResult>) -> FileLoad {
        FileLoad { config_result }
    }

    pub fn start(&self) {
        let images = self.config_result.images.clone();
        log::info(&format!("start load {} images", images.len()));
    }
}
