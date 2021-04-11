use std::fs::File;
use std::io::Write;

pub fn write_to_ppm(buffer: Vec<u8>, path: String, width: i32, height: i32) {
    match File::create(path) {
        Ok(mut f) => {
            let t = f.write(format!("P6\n{} {}\n255\n", width, height).as_bytes());
        }
        Err(err) => {
            println!("write_to_ppm err: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::msg_const::PROJ_DIR;

    #[test]
    fn test_write_to_ppm() {
        write_to_ppm(Vec::new(), format!("{}/target/out.ppm", PROJ_DIR), 5, 5);
    }
}
