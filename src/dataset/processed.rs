use std::fs;
use crate::config::Config;
use crate::error::Result;

/// 创建新的数据预处理方案（已加入幂等性支持）
pub fn create_processed_scheme(cfg: &Config, dataset_name: &str, scheme_name: &str) -> Result<()> {
    let base = cfg
        .data_root
        .join(dataset_name)
        .join("processeddata")
        .join(scheme_name);

    // --- 幂等性逻辑开始 ---
    // 如果该方案的目录已经存在，我们认为它已经创建过了
    if base.exists() {
        println!("⏭️  数据集 {} 的方案 {} 已存在，跳过创建", dataset_name, scheme_name);
        return Ok(());
    }
    // --- 幂等性逻辑结束 ---

    // 创建目录
    fs::create_dir_all(base.join("data"))?;

    // 创建占位文档
    fs::write(base.join("README.md"), format!("# 预处理方案: {}\n", scheme_name))?;
    fs::write(base.join("process_doc.md"), "# 数据处理与划分说明\n")?;

    println!("✅ 数据集 {} 预处理方案 {} 已创建", dataset_name, scheme_name);
    Ok(())
}

/// 批量创建预处理方案
pub fn create_multiple_processed_schemes(
    cfg: &Config,
    dataset_name: &str,
    schemes: Vec<&str>,
) -> Result<()> {
    for s in schemes {
        create_processed_scheme(cfg, dataset_name, s)?;
    }
    Ok(())
}
