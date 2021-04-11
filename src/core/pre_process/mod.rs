use glob::glob;
use serde_json::{self, Error};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

mod config_model;
mod config_result;

pub use crate::core::pre_process::config_model::ConfigModel;
pub use crate::core::pre_process::config_result::ConfigResult;

use crate::utils::log;

pub fn pre_process(config_path: &str) -> ConfigResult {
    log::info("start reading configuration");

    match File::open(config_path) {
        Ok(mut file) => {
            let mut content = String::new();

            if let Err(err) = file.read_to_string(&mut content) {
                log::error(&format!("read config file error: {}", err));
                return ConfigResult {
                    images: Arc::new(vec![]),
                    gui: true,
                };
            }

            log::info("read the configuration successfully");

            parse_config(content)
        }
        Err(_) => {
            log::error(&format!("config file: \"{}\" can not open", config_path));
            ConfigResult {
                images: Arc::new(vec![]),
                gui: true,
            }
        }
    }
}

fn parse_config(content: String) -> ConfigResult {
    let cfg_data: Result<ConfigModel, Error> = serde_json::from_str(&content);
    let cfg_data = match cfg_data {
        Ok(cfg_data) => cfg_data,
        Err(err) => {
            log::error(&format!("parse config json error: {}", err));
            return ConfigResult {
                images: Arc::new(vec![]),
                gui: true,
            };
        }
    };

    log::info("parse the configuration successfully");

    ConfigResult {
        images: Arc::new(scan_images(&cfg_data)),
        gui: cfg_data.gui,
    }
}

fn scan_images(cfg: &ConfigModel) -> Vec<String> {
    let root_path = &cfg.image_folder_root_path;
    let recursion = cfg.recursion;

    let p_jpg = format!("{}{}/*.jpg", root_path, if recursion { "/**" } else { "" });
    let p_jpeg = format!("{}{}/*.jpeg", root_path, if recursion { "/**" } else { "" });
    let p_png = format!("{}{}/*.png", root_path, if recursion { "/**" } else { "" });

    let mut result: Vec<String> = Vec::new();

    for entry in glob(&p_jpg)
        .unwrap()
        .chain(glob(&p_jpeg).unwrap())
        .chain(glob(&p_png).unwrap())
    {
        match entry {
            Ok(path) => match path.into_os_string().into_string() {
                Ok(path_string) => result.push(path_string),
                Err(_) => log::warning(&"failed to get PathBuf".to_string()),
            },
            Err(err) => log::error(&format!("glob error: {}", err)),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn test_parse_config() {
        let invalid = format!("");
        let result = parse_config(invalid);

        assert_eq!(result.images.len(), 0);

        let mut valid =
            "{\"image_folder_root_path\": \"PLACEHOLDER/tests/image\", \"recursion\": true}"
                .to_string();
        valid = valid.replace("PLACEHOLDER", MANIFEST_DIR);
        let result = parse_config(valid.clone());

        assert_eq!(result.images.len(), 4);
    }
}
