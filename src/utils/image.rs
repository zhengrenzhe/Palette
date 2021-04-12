use std::fs::File;
use std::io::Write;

/// this function only used to test whether image decode is correct
#[allow(dead_code)]
pub fn write_to_ppm(buffer: Vec<u8>, path: String, width: usize, height: usize) {
    match File::create(path) {
        Ok(mut f) => {
            let header = format!("P6\n{} {}\n255\n", width, height);
            f.write_all(header.as_bytes()).unwrap();
            for y in 0..height {
                for x in (0..(width * 3)).step_by(3) {
                    let base = y * width * 3 + x;
                    f.write_all(&[*buffer.get(base).unwrap()]).unwrap();
                    f.write_all(&[*buffer.get(base + 1).unwrap()]).unwrap();
                    f.write_all(&[*buffer.get(base + 2).unwrap()]).unwrap();
                }
            }
        }
        Err(err) => {
            println!("write_to_ppm err: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::image_decode::decode;
    use crate::utils::file::read;
    use crate::utils::msg_const::PROJ_DIR;
    use std::convert::TryFrom;

    #[test]
    fn test_write_to_ppm() {
        write_to_ppm(
            vec![1, 2, 3, 4, 5, 6],
            format!("{}/target/out.ppm", PROJ_DIR),
            2,
            1,
        );
    }

    #[test]
    fn test_write_image_to_ppm() {
        let f = read(&format!("{}/tests/image/a.png", PROJ_DIR)).unwrap();
        let d = decode(f.buffer).unwrap();
        write_to_ppm(
            d.buffer,
            format!("{}/target/a.ppm", PROJ_DIR),
            usize::try_from(d.width).unwrap(),
            usize::try_from(d.height).unwrap(),
        )
    }
}
