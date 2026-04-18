//! 数据库错误处理模块

use thiserror::Error;

/// 数据库操作错误类型
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// DuckDB 数据库错误
    #[error("DuckDB error: {0}")]
    DuckDB(#[from] duckdb::Error),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO 错误
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// TOML 解析错误
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// 连接未初始化
    #[error("Connection not initialized")]
    NotInitd,

    /// 路径未找到
    #[error("Path not found: {0}")]
    PathNotFound(String),

    /// 无效的配置
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// 数据库操作结果类型
pub type DatabaseResult<T> = Result<T, DatabaseError>;
