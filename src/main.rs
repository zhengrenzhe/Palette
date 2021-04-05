use clap::{App, Arg};

mod commands;
mod core;
mod utils;

use crate::core::scheduler::Scheduler;
use crate::utils::log;
use crate::utils::msg_const;

fn main() {
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

    match matches.value_of("config") {
        Some(config_path) => start(config_path),
        None => {}
    }
}

fn start(config_path: &str) {
    let cfg = core::pre_process::pre_process(config_path);
    if cfg.images.len() == 0 {
        return log::warning(
            "there are no jpg, png, jpeg files in the specified directory, process will stop",
        );
    }

    log::success(&format!("The preparation work has been completed, there are currently {} images waiting to be calculated", cfg.images.len()));

    Scheduler::new(cfg).start();
}
