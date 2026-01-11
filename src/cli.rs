use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dataset-manager")]
#[command(about = "一个简单的数据集管理工具", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", help = "指定配置文件路径")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 初始化原始数据集
    Init {
        #[arg(help = "数据集名称")]
        name: String,
    },
    /// 创建预处理方案
    Process {
        #[arg(help = "数据集名称")]
        dataset: String,
        #[arg(help = "方案名称")]
        scheme: String,
    },
    /// 为用户软链接数据
    Link {
        #[arg(help = "数据集名称")]
        dataset: String,
        #[arg(help = "方案名称")]
        scheme: String,
        #[arg(help = "用户工作目录")]
        workdir: String,
    },
}
