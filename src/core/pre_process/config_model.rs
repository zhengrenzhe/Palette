use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigModel {
    /// image folder root path
    pub image_folder_root_path: String,

    /// recursion find images in child folders
    #[serde(default = "default_recursion")]
    pub recursion: bool,

    /// output log with gui
    #[serde(default = "default_gui")]
    pub gui: bool,
}

fn default_recursion() -> bool {
    true
}

fn default_gui() -> bool {
    true
}
