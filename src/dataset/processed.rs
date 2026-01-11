use std::fs;

use crate::config::Config;
use crate::error::Result;
/// 创建新的数据预处理方案
pub fn create_processed_scheme(
    cfg: &Config,
    dataset_name: &str,
    scheme_name: &str,
) -> Result<()> {
    let base = cfg
        .data_root
        .join(dataset_name)
        .join("processeddata")
        .join(scheme_name);

    fs::create_dir_all(base.join("data"))?;

    fs::write(base.join("README.md"), "# 预处理方案简评\n")?;
    fs::write(base.join("process_doc.md"), "# 数据处理与划分说明\n")?;

    println!("✅ 预处理方案 {} 已创建", scheme_name);
    Ok(())
}
