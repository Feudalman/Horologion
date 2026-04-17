//! 数据库配置管理模块
//!
//! 根据不同的运行模式提供数据库路径配置：
//! - 测试模式：内存数据库
//! - 开发模式：项目根目录的 playground/db
//! - 生产模式：系统数据目录或配置文件指定的目录

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// 数据库路径
    pub path: String,
    /// 是否为内存数据库
    pub is_memory: bool,
}

impl DatabaseConfig {
    /// 创建一个新的数据库配置
    pub fn new() -> Self {
        let path = std::env::var("DATABASE_URL").unwrap_or_else(|_| ":memory:".to_string());
        Self {
            path,
            is_memory: cfg!(test),
        }
    }
}
