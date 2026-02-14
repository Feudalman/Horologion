//! Horologion 数据库模块
//! 
//! 提供 DuckDB 数据库的连接管理、表结构定义和数据操作功能

pub mod config;
pub mod connection;
pub mod schema;
pub mod operations;

// 重新导出主要类型和函数
pub use config::{DatabaseConfig, RunMode};
pub use connection::{DatabaseManager, DatabaseError, DatabaseResult};
pub use schema::{InputEvent, WindowRecord, AppUsageStats, SchemaManager};
pub use operations::{InputEventOps, WindowRecordOps, AppUsageStatsOps};

/// 数据库初始化函数
/// 
/// 这是一个便捷函数，用于快速初始化数据库连接和表结构
pub fn initialize_database() -> DatabaseResult<DatabaseManager> {
    let db = DatabaseManager::from_env()?;
    db.initialize()?;
    SchemaManager::initialize_tables(&db)?;
    Ok(db)
}

/// 数据库初始化函数（指定配置）
pub fn initialize_database_with_config(config: DatabaseConfig) -> DatabaseResult<DatabaseManager> {
    let db = DatabaseManager::new(config);
    db.initialize()?;
    SchemaManager::initialize_tables(&db)?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let db = initialize_database_with_config(config).unwrap();
        assert!(db.is_initialized());
    }
}
