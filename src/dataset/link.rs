use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::AppError;
use crate::error::Result;

/// 使用某个预处理方案（创建符号链接，支持幂等性）
pub fn link_scheme_to_user(
    cfg: &Config,
    dataset_name: &str,
    scheme_name: &str,
    user_workdir: &str,
) -> Result<()> {
    // 1. 定义源路径与目标链接路径
    let target = cfg
        .data_root
        .join(dataset_name)
        .join("processeddata")
        .join(scheme_name);

    let link_path = PathBuf::from(user_workdir)
        .join(dataset_name)
        .join(scheme_name);

    // 2. 检查源数据是否存在（卫语句）
    if !target.exists() {
        // 逻辑优化：直接检查最终 target 即可推断 dataset 或 scheme 是否缺失
        return Err(AppError::SchemeNotFound(
            scheme_name.to_string(),
            dataset_name.to_string(),
        ));
    }

    // 3. 幂等性处理：处理已存在的链接
    if fs::symlink_metadata(&link_path).is_ok() {
        // 获取当前链接指向的实际路径
        if let Ok(existing_target) = fs::read_link(&link_path) {
            if existing_target == target {
                println!("⏭️  链接已存在且指向正确: {:?}", link_path);
                return Ok(());
            }
        }
        // 如果指向错误或者是一个普通文件，则先删除它以便重建
        fs::remove_file(&link_path)?;
        println!("♻️  清理旧的无效链接: {:?}", link_path);
    }

    // 4. 创建父目录并建立链接
    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    symlink(target, link_path)?;

    println!("🔗 已为用户创建数据集 {} 方案 {} 的软链接", dataset_name, scheme_name);
    Ok(())
}

pub fn link_multiple_schemes(
    cfg: &Config,
    dataset: &str,
    schemes: Vec<&str>,
    user_dir: &str,
) -> Result<()> {
    for s in schemes {
        link_scheme_to_user(cfg, dataset, s, user_dir)?;
    }
    Ok(())
}
