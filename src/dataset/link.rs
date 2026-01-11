use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::AppError;
use crate::error::Result;
/// 使用某个预处理方案（创建符号链接）
pub fn link_scheme_to_user(
    cfg: &Config,
    dataset_name: &str,
    scheme_name: &str,
    user_workdir: &str,
) -> Result<()> {
    let target = cfg
        .data_root
        .join(dataset_name)
        .join("processeddata")
        .join(scheme_name);

    let dataset_path = cfg.data_root.join(dataset_name);
    let scheme_path = dataset_path.join("processeddata").join(scheme_name);
    if !fs::exists(&dataset_path)? {
        return Err(AppError::DatasetNotFound(dataset_name.to_string()));
    } else if !fs::exists(&scheme_path)? {
        return Err(AppError::SchemeNotFound(
            dataset_name.to_string(),
            scheme_name.to_string(),
        ));
    }
    let link_path = PathBuf::from(user_workdir)
        .join(dataset_name)
        .join(scheme_name);

    fs::create_dir_all(link_path.parent().unwrap())?;
    symlink(target, link_path)?;

    println!("🔗 已为用户创建数据集{}预处理方案{}链接",dataset_name,scheme_name);
    Ok(())
}

#[allow(dead_code)]
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
