use crate::core::image_decode::DecodedImage;

pub struct ImageRaw {
    pub path: String,
    pub decoded: DecodedImage,
}

impl ImageRaw {
    pub fn new(decoded: DecodedImage, path: String) -> ImageRaw {
        ImageRaw { decoded, path }
    }
}
