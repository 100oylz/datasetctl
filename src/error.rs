use thiserror::Error;

// 定义全局通用的 Result 别名
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("文件系统操作失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("配置项缺失: {0}")]
    ConfigError(String),

    #[error("数据集 {0} 不存在")]
    DatasetNotFound(String),

    #[error("数据集[{0}]中未知的预处理方案: {1} ")]
    SchemeNotFound(String,String),
}