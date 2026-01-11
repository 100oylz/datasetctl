use std::fs;

use crate::config::Config;
use crate::error::Result;

/// 创建原始数据集结构
pub fn create_raw_dataset(cfg: &Config, dataset_name: &str) -> Result<()> {
    let base = cfg.data_root.join(dataset_name).join("rawdata");

    fs::create_dir_all(base.join("data"))?;

    // 创建占位文档
    fs::write(base.join("README.md"), "# 数据集简评\n")?;
    fs::write(base.join("document.md"), "# 数据集完整说明\n")?;

    println!("✅ 原始数据集结构已创建");
    Ok(())
}