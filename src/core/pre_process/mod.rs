use glob::glob;
use serde::Deserialize;
use serde_json::{self, Error};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

pub mod config_model;
pub mod config_result;

use crate::core::pre_process::config_model::ConfigModel;
use crate::core::pre_process::config_result::ConfigResult;
use crate::utils::log;

pub fn pre_process(config_path: &str) -> ConfigResult {
    log::info("start reading configuration");

    match File::open(config_path) {
        Ok(mut file) => {
            let mut content = String::new();

            match file.read_to_string(&mut content) {
                Err(err) => {
                    log::error(&format!("read config file error: {}", err));
                    return ConfigResult { images: vec![] };
                }
                _ => {}
            }

            log::info("read the configuration successfully");

            let cfg_data: Result<ConfigModel, Error> = serde_json::from_str(&content);
            let cfg_data = match cfg_data {
                Ok(cfg_data) => cfg_data,
                Err(err) => {
                    log::error(&format!("parse config json error: {}", err));
                    return ConfigResult { images: vec![] };
                }
            };

            log::info("parse the configuration successfully");

            let images = scan_images(cfg_data);

            ConfigResult { images }
        }
        Err(_) => {
            log::error(&format!("config file: \"{}\" can not open", config_path));
            return ConfigResult { images: vec![] };
        }
    }
}

fn scan_images(cfg: ConfigModel) -> Vec<String> {
    let root_path = cfg.image_folder_root_path;
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
                Err(_) => log::warning(&format!("failed to get PathBuf")),
            },
            Err(err) => log::error(&format!("glob error: {}", err)),
        }
    }

    result
}
