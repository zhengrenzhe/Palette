use clap::{App, Arg};

mod commands;

use commands::single::get_single_dominant_color;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let single = Arg::with_name("single")
        .long("single")
        .value_name("IMAGE PATH")
        .help("calculate single image dominant color")
        .takes_value(true);

    let matches = App::new(PKG_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .args(&vec![single])
        .get_matches();

    match matches.value_of("single") {
        Some(path) => return get_single_dominant_color(path),
        None => {}
    }
}
