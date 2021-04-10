use image::io::Reader;
use image::ImageError;

use std::io::Cursor;

// decode is very slow in non-release mode
pub fn decode(buffer: Vec<u8>) -> Result<Vec<u8>, ImageError> {
    let d = Reader::new(Cursor::new(buffer.as_slice()))
        .with_guessed_format()
        .unwrap();

    match d.decode() {
        Ok(img) => Ok(img.to_bytes()),
        Err(err) => Err(err),
    }
}
