use crossbeam::queue::ArrayQueue;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};

use crate::core::scheduler::SchedulerChannelMsg;
use crate::utils::file::ReadedFile;
use crate::utils::log;

pub struct ImageParse {
    file_loaded_queue: Arc<RwLock<ArrayQueue<ReadedFile>>>,
}

impl ImageParse {
    pub fn new(file_loaded_queue: Arc<RwLock<ArrayQueue<ReadedFile>>>) -> ImageParse {
        ImageParse { file_loaded_queue }
    }

    pub fn start(&self) {
        log::info("create image parse thread");
        // for msg in receiver {
        //     match msg {
        //         SchedulerChannelMsg::ImageLoaded => {
        //             match self.file_loaded_queue.read().unwrap().pop() {
        //                 None => log::info("no file"),
        //                 Some(_) => log::info("got file"),
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }
}
