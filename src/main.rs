use clap::{App, Arg};

mod commands;
mod core;
mod utils;

use crate::utils::log;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let config = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE PATH")
        .help("read config")
        .takes_value(true);

    let matches = App::new(PKG_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .args(&vec![config])
        .get_matches();

    match matches.value_of("config") {
        Some(config_path) => start(config_path),
        None => {}
    }
}

fn start(config_path: &str) {
    let res = core::pre_process::pre_process(config_path);
    if res.images.len() == 0 {
        return log::warning(
            "there are no jpg, png, jpeg files in the specified directory, process will stop",
        );
    }

    log::success(&format!("The preparation work has been completed, there are currently {} images waiting to be calculated", res.images.len()))
}
