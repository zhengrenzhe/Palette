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

#[cfg(test)]
mod tests {
    use crate::core::image_decode::decode;
    use crate::utils::file::read;
    use crate::utils::image::write_to_ppm;
    use crate::utils::msg_const::PROJ_DIR;
    use std::convert::TryFrom;
    use std::fs::File;

    #[test]
    fn test_png_decode() {
        let f = read(&format!("{}/tests/image/a.png", PROJ_DIR)).unwrap();
        let d = decode(f.buffer).unwrap();
        let output = format!("{}/target/a.ppm", PROJ_DIR);
        write_to_ppm(
            d.buffer,
            output.clone(),
            usize::try_from(d.width).unwrap(),
            usize::try_from(d.height).unwrap(),
        );
        let f = File::open(output).unwrap();
        // if assert is true, decode png is correct
        assert_eq!(f.metadata().unwrap().len(), 5674049);
    }

    #[test]
    fn test_jpg_decode() {
        let f = read(&format!("{}/tests/image/b.jpg", PROJ_DIR)).unwrap();
        let d = decode(f.buffer).unwrap();
        let output = format!("{}/target/b.ppm", PROJ_DIR);
        write_to_ppm(
            d.buffer,
            output.clone(),
            usize::try_from(d.width).unwrap(),
            usize::try_from(d.height).unwrap(),
        );
        let f = File::open(output).unwrap();
        // if assert is true, decode jpg is correct
        assert_eq!(f.metadata().unwrap().len(), 20476817);
    }
}
