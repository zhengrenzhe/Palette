use image::io::Reader;
use image::GenericImageView;
use std::io::{Cursor, Error, ErrorKind};

pub struct DecodedImage {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl DecodedImage {
    pub fn new(buffer: Vec<u8>, width: u32, height: u32) -> DecodedImage {
        DecodedImage {
            buffer,
            width,
            height,
        }
    }
}

/// decode is very slow in non-release mode
///
/// buffer: image file raw buffer
/// return: image color raw bugger
pub fn decode(buffer: Vec<u8>) -> Result<DecodedImage, Error> {
    match Reader::new(Cursor::new(buffer.as_slice())).with_guessed_format() {
        Ok(img) => match img.decode() {
            Ok(img) => Ok(DecodedImage::new(img.to_bytes(), img.width(), img.height())),
            Err(err) => Err(Error::new(ErrorKind::Other, err)),
        },
        Err(err) => Err(Error::new(ErrorKind::Other, err)),
    }
}
