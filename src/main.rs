mod config;
mod error;
mod cli;
mod dataset;



use clap::Parser;
use crate::error::Result;


fn main() -> Result<()> {
    // 1. 解析命令行参数
    let cli = cli::Cli::parse();

    // 2. 初始化配置 (假设 config 模块有 load 方法)
    let cfg = config::Config::load(cli.config.as_deref())?;

    // 3. 匹配子命令并调用 dataset 模块
    match cli.command {
        cli::Commands::Init { name } => {
            dataset::raw::create_raw_dataset(&cfg, &name)?;
        }
        cli::Commands::Process { dataset, scheme } => {
            dataset::processed::create_processed_scheme(&cfg, &dataset, &scheme)?;
        }
        cli::Commands::Link { dataset, scheme, workdir } => {
            dataset::link::link_scheme_to_user(&cfg, &dataset, &scheme, &workdir)?;
        }
    }

    Ok(())
}
