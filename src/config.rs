use std::path::PathBuf;
use crate::error::Result;
use crate::error::AppError;
/// 全局配置
pub struct Config {
    pub data_root: PathBuf,
}

impl Config {

    pub fn new(data_root: PathBuf) -> Self {
        Self { data_root }
    }
    
    pub fn load(_path: Option<&str>) -> Result<Self> {
        // 1. 获取当前用户的主目录 (Home Directory)
        let home_dir = home::home_dir().ok_or_else(|| {
            AppError::ConfigError("无法找到用户主目录".to_string())
        })?;

        // 2. 拼接成 ~/data
        let data_root = home_dir.join("data").join("dataset");

        // 3. 自动创建该目录（如果不存在）
        if !data_root.exists() {
            std::fs::create_dir_all(&data_root)?;
            println!("📁 已自动创建数据根目录: {:?}", data_root);
        }

        Ok(Config::new ( data_root ))
    }

}