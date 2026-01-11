use std::fs;
use crate::config::Config;
use crate::error::Result;

/// 创建原始数据集结构（已加入幂等性支持）
pub fn create_raw_dataset(cfg: &Config, dataset_name: &str) -> Result<()> {
    let base = cfg.data_root.join(dataset_name).join("rawdata");

    // --- 幂等性检查 ---
    // 如果 rawdata 目录已存在，说明结构已经初始化过
    if base.exists() {
        println!("⏭️  原始数据集 {} 结构已存在，跳过创建", dataset_name);
        return Ok(());
    }
    // ------------------

    // 创建核心目录
    fs::create_dir_all(base.join("data"))?;

    // 创建占位文档，使用 format! 注入数据集名称使文档更有意义
    fs::write(
        base.join("README.md"), 
        format!("# 数据集 {} 简评\n", dataset_name)
    )?;
    fs::write(
        base.join("document.md"), 
        format!("# 数据集 {} 完整说明\n", dataset_name)
    )?;

    println!("✅ 原始数据集 {} 结构已创建", dataset_name);
    Ok(())
}

/// 批量创建原始数据集结构
pub fn create_multiple_raw_datasets(cfg: &Config, datasets: Vec<&str>) -> Result<()> {
    for d in datasets {
        create_raw_dataset(cfg, d)?;
    }
    Ok(())
}
