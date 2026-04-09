//! 数据库模块
//!
//! 提供数据库连接管理、配置管理和路径解析功能

pub mod error;
pub mod settings;
pub mod paths;
pub mod config;
pub mod connection;

// 重导出主要类型
pub use error::{DatabaseError, DatabaseResult};
pub use settings::{AppSettings, DatabaseSettings, LoggingSettings, PerformanceSettings, ListenerSettings};
pub use paths::RunMode;
pub use config::DatabaseConfig;
pub use connection::DatabaseManager;

/// 便捷函数：初始化数据库
///
/// 根据环境变量自动配置并初始化数据库连接
///
/// # Example
///
/// ```no_run
/// use database::db::initialize_database;
///
/// let db = initialize_database().expect("Failed to initialize database");
/// ```
pub fn initialize_database() -> DatabaseResult<DatabaseManager> {
    let db = DatabaseManager::from_env()?;
    db.initialize()?;
    Ok(db)
}

/// 便捷函数：使用自定义配置初始化数据库
///
/// # Example
///
/// ```no_run
/// use database::db::{initialize_database_with_config, DatabaseConfig, RunMode};
///
/// let config = DatabaseConfig::new(RunMode::Development).unwrap();
/// let db = initialize_database_with_config(config).expect("Failed to initialize database");
/// ```
pub fn initialize_database_with_config(config: DatabaseConfig) -> DatabaseResult<DatabaseManager> {
    let db = DatabaseManager::new(config);
    db.initialize()?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_database() {
        std::env::set_var("DATABASE_URL", ":memory:");
        let result = initialize_database();
        assert!(result.is_ok());

        let db = result.unwrap();
        assert!(db.is_initialized());

        std::env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_initialize_database_with_config() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let result = initialize_database_with_config(config);
        assert!(result.is_ok());

        let db = result.unwrap();
        assert!(db.is_initialized());
    }
}
