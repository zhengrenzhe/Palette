use clap::{App, Arg};
use std::panic;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

mod commands;
mod core;
mod utils;

use crate::core::scheduler::state::PALETTE_STATE;
use crate::core::scheduler::Scheduler;
use crate::utils::log;
use crate::utils::msg_const;

fn main() {
    // catch all panic error
    panic::set_hook(Box::new(|info| log::error(&format!("{}", info))));

    let config = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE PATH")
        .help("read config")
        .takes_value(true);

    let matches = App::new(msg_const::PKG_NAME)
        .version(msg_const::VERSION)
        .author(msg_const::AUTHOR)
        .about(msg_const::DESCRIPTION)
        .args(&vec![config])
        .get_matches();

    if let Some(config_path) = matches.value_of("config") {
        start(config_path)
    }
}

fn start(config_path: &str) {
    let cfg = core::pre_process::pre_process(config_path);

    PALETTE_STATE.write().unwrap().set_gui(cfg.gui);

    if cfg.images.is_empty() {
        return log::warning(
            "there are no jpg, png, jpeg files in the specified directory, process will stop",
        );
    }

    log::success(&format!("The preparation work has been completed, there are currently {} images waiting to be calculated", cfg.images.len()));

    let mut hs: Vec<JoinHandle<()>> = vec![];

    if cfg.gui {
        hs.push(thread::spawn(|| {
            log::gui::draw();
        }))
    }

    let gui = cfg.gui;
    Scheduler::new(Arc::new(cfg)).start();

    if gui {
        for h in hs {
            log::info("all jobs has finished, press q to exit palette");
            h.join().unwrap();
        }
    } else {
        log::info("all jobs has finished, palette will auto exited");
    }
}
