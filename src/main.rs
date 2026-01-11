mod cli;
mod config;
mod dataset;
mod error;

use crate::error::Result;
use clap::Parser;

fn main() -> Result<()> {
    // 1. 解析命令行参数
    let cli = cli::Cli::parse();

    // 2. 初始化配置 (假设 config 模块有 load 方法)
    let cfg = config::Config::load(cli.config.as_deref())?;

    // 3. 匹配子命令并调用 dataset 模块
    match cli.command {
        cli::Commands::Init { name } => {
            // 解析：逗号分割 -> 去空格 -> 过滤空值
            let datasets: Vec<&str> = name
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if datasets.is_empty() {
                return Err(crate::error::AppError::ConfigError(
                    "未提供有效的数据集名称，请使用如 'mnist,cifar10' 的格式".into(),
                ));
            }

            // 统一调用批量创建函数
            dataset::raw::create_multiple_raw_datasets(&cfg, datasets)?;
        }
        // src/main.rs 里的 match cli.command 对应部分
        cli::Commands::Process { dataset, scheme } => {
            // 解析：逗号分割 -> 去空格 -> 过滤空值
            let schemes: Vec<&str> = scheme
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if schemes.is_empty() {
                return Err(crate::error::AppError::ConfigError(
                    "未提供有效的方案名称，请使用如 'v1,v2' 的格式".into(),
                ));
            }

            // 统一调用批量创建函数
            dataset::processed::create_multiple_processed_schemes(&cfg, &dataset, schemes)?;
        }
        cli::Commands::Link {
            dataset,
            scheme,
            workdir,
        } => {
            // 1. 解析字符串：按逗号分割 -> 去空格 -> 过滤空值 -> 收集为 Vec<&str>
            let schemes: Vec<&str> = scheme
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            // 2. 安全检查：如果分割后什么都没有，报错
            if schemes.is_empty() {
                return Err(crate::error::AppError::ConfigError(
                    "未提供有效的 Scheme 名称，请输入如 'v1' 或 'v1,v2'".into(),
                ));
            }

            // 3. 统一调用多方案处理函数
            dataset::link::link_multiple_schemes(&cfg, &dataset, schemes, &workdir)?;
        }
    }

    Ok(())
}
