use crate::core::image_decode::ImageRaw;

pub fn get_dominant_colors(raw: &ImageRaw) {}

#[cfg(test)]
mod tests {
    use crate::utils::msg_const::PROJ_DIR;

    #[test]
    fn test_get_dominant_colors() {
        let path = format!("{}/tests/image/b.jpg", PROJ_DIR);
        let img = image::io::Reader::open(path).unwrap().decode().unwrap();
        let bytes = img.to_bytes();
        println!("{}", bytes.len());
    }
}
