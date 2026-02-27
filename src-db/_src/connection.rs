//! 数据库连接管理模块

use crate::config::DatabaseConfig;
use duckdb::{Connection, Result as DuckResult};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("DuckDB error: {0}")]
    DuckDB(#[from] duckdb::Error),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Connection not initialized")]
    NotInitialized,
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// 数据库连接管理器
pub struct DatabaseManager {
    connection: Arc<Mutex<Option<Connection>>>,
    config: DatabaseConfig,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
            config,
        }
    }

    /// 从环境变量创建数据库管理器
    pub fn from_env() -> DatabaseResult<Self> {
        let config = DatabaseConfig::from_env()
            .map_err(|e| DatabaseError::Config(e.to_string()))?;
        Ok(Self::new(config))
    }

    /// 初始化数据库连接
    pub fn initialize(&self) -> DatabaseResult<()> {
        // 确保数据库目录存在
        self.config.ensure_db_directory()
            .map_err(|e| DatabaseError::Config(e.to_string()))?;

        // 创建连接
        let conn = Connection::open(&self.config.connection_string)?;
        
        // 设置一些优化参数
        self.configure_connection(&conn)?;
        
        // 存储连接
        let mut connection_guard = self.connection.lock().unwrap();
        *connection_guard = Some(conn);
        
        log::info!("Database initialized: {}", self.config.connection_string);
        Ok(())
    }

    /// 配置数据库连接参数
    fn configure_connection(&self, conn: &Connection) -> DatabaseResult<()> {
        // 启用 WAL 模式（如果不是内存数据库）
        if !self.config.is_memory {
            conn.execute_batch("PRAGMA journal_mode = WAL;")?;
        }
        
        // 设置其他优化参数
        conn.execute_batch("
            PRAGMA synchronous = NORMAL;
            PRAGMA cache_size = 1000000;
            PRAGMA temp_store = memory;
            PRAGMA mmap_size = 268435456;
        ")?;
        
        Ok(())
    }

    /// 执行带有连接的闭包
    pub fn with_connection<F, R>(&self, f: F) -> DatabaseResult<R>
    where
        F: FnOnce(&Connection) -> DatabaseResult<R>,
    {
        let connection_guard = self.connection.lock().unwrap();
        let conn = connection_guard
            .as_ref()
            .ok_or(DatabaseError::NotInitialized)?;
        f(conn)
    }

    /// 获取数据库配置
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// 检查连接是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.connection.lock().unwrap().is_some()
    }

    /// 关闭数据库连接
    pub fn close(&self) -> DatabaseResult<()> {
        let mut connection_guard = self.connection.lock().unwrap();
        if let Some(conn) = connection_guard.take() {
            // DuckDB 的 Connection 会在 drop 时自动关闭
            drop(conn);
            log::info!("Database connection closed");
        }
        Ok(())
    }

    /// 执行 SQL 语句（无返回值）
    pub fn execute(&self, sql: &str) -> DatabaseResult<usize> {
        self.with_connection(|conn| {
            let affected = conn.execute(sql, [])?;
            Ok(affected)
        })
    }

    /// 执行批量 SQL 语句
    pub fn execute_batch(&self, sql: &str) -> DatabaseResult<()> {
        self.with_connection(|conn| {
            conn.execute_batch(sql)?;
            Ok(())
        })
    }
}

// 实现 Clone，以便在多个地方使用同一个数据库管理器
impl Clone for DatabaseManager {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseConfig, RunMode};

    #[test]
    fn test_database_manager_memory() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);
        
        assert!(!manager.is_initialized());
        
        manager.initialize().unwrap();
        assert!(manager.is_initialized());
        
        // 测试简单查询
        let result = manager.with_connection(|conn| {
            let mut stmt = conn.prepare("SELECT 1 as test")?;
            let rows: Vec<i32> = stmt.query_map([], |row| {
                Ok(row.get(0)?)
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        }).unwrap();
        
        assert_eq!(result, vec![1]);
    }
}
