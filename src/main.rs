use clap::{App, Arg};

mod commands;
mod core;
mod utils;

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
        Some(path) => println!("{}", path),
        None => {}
    }
}
