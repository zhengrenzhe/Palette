use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigModel {
    /// 图片文件夹根目录
    pub image_folder_root_path: String,

    /// 是否递归查找子目录中的图片
    pub recursion: bool,
}
