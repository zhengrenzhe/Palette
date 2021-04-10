use std::thread;
use std::thread::Builder;

// create new thread with name
pub fn new_thread(name: &str) -> Builder {
    thread::Builder::new().name(name.to_string())
}
