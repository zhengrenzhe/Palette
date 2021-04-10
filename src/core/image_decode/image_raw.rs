pub struct ImageRaw {
    pub bytes: Vec<u8>,
    pub path: String,
}

impl ImageRaw {
    pub fn new(bytes: Vec<u8>, path: String) -> ImageRaw {
        ImageRaw { bytes, path }
    }
}
