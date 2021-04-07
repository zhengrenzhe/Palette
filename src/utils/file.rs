use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use crate::utils::log;

pub struct ReadedFile {
    pub descriptor: Arc<File>,
    pub buffer: Arc<Vec<u8>>,
}

pub fn read(path: &str) -> Option<ReadedFile> {
    match File::open(path) {
        Ok(mut file) => {
            let mut buf: Vec<u8> = vec![];

            if file.read_to_end(&mut buf).is_ok() {
                log::success(&format!("read file {} success", path));
                return Some(ReadedFile {
                    descriptor: Arc::new(file),
                    buffer: Arc::new(buf),
                });
            }

            log::error(&format!("read file {} error", path));
            None
        }
        Err(err) => {
            log::error(&format!("open file {} error: {}", path, err));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn test_load() {
        let valid_path = format!("{}/tests/image/a.png", MANIFEST_DIR);
        match read(&valid_path) {
            None => panic!(),
            Some(f) => assert_eq!(f.buffer.len(), 1061900),
        };
        match read("invalid_path") {
            None => {}
            Some(_) => panic!(),
        };
    }
}
