//! 数据库连接管理模块

use crate::db::config::DatabaseConfig;
use crate::db::error::{DatabaseError, DatabaseResult};
use duckdb::Connection;
use std::sync::{Arc, Mutex};

/// 数据库连接管理器
///
/// 使用 Arc<Mutex<Option<Connection>>> 提供线程安全的连接管理
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
        let config = DatabaseConfig::from_env()?;
        Ok(Self::new(config))
    }

    /// 初始化数据库连接
    pub fn initialize(&self) -> DatabaseResult<()> {
        // 确保数据库目录存在
        self.config.ensure_db_directory()?;

        // 创建连接
        let conn = Connection::open(&self.config.connection_string)?;

        // 配置连接参数
        self.configure_connection(&conn)?;

        // 存储连接
        let mut connection_guard = self.connection.lock().unwrap();
        *connection_guard = Some(conn);

        log::info!(
            "Database initialized: {} (memory: {})",
            self.config.connection_string,
            self.config.is_memory
        );
        Ok(())
    }

    /// 配置数据库连接参数
    ///
    /// 应用性能优化设置
    fn configure_connection(&self, conn: &Connection) -> DatabaseResult<()> {
        let perf = &self.config.performance;

        // 构建 PRAGMA 语句
        let mut pragmas = Vec::new();

        // 日志模式（如果不是内存数据库）
        if !self.config.is_memory {
            if let Some(ref journal_mode) = perf.journal_mode {
                pragmas.push(format!("PRAGMA journal_mode = {};", journal_mode));
            }
        }

        // 同步模式
        if let Some(ref synchronous) = perf.synchronous {
            pragmas.push(format!("PRAGMA synchronous = {};", synchronous));
        }

        // 缓存大小
        if let Some(cache_size) = perf.cache_size {
            pragmas.push(format!("PRAGMA cache_size = {};", cache_size));
        }

        // 内存映射大小
        if let Some(mmap_size) = perf.mmap_size {
            pragmas.push(format!("PRAGMA mmap_size = {};", mmap_size));
        }

        // 临时存储使用内存
        pragmas.push("PRAGMA temp_store = memory;".to_string());

        // 批量执行 PRAGMA 语句
        if !pragmas.is_empty() {
            let pragma_batch = pragmas.join("\n");
            log::debug!("Applying database PRAGMAs:\n{}", pragma_batch);
            conn.execute_batch(&pragma_batch)?;
        }

        Ok(())
    }

    /// 执行带有连接的闭包
    ///
    /// 提供对数据库连接的安全访问
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
// 这是一个浅拷贝，多个 DatabaseManager 实例共享同一个连接
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
    use crate::db::paths::RunMode;

    #[test]
    fn test_database_manager_memory() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);

        assert!(!manager.is_initialized());

        manager.initialize().unwrap();
        assert!(manager.is_initialized());

        // 测试简单查询
        let result = manager
            .with_connection(|conn| {
                let mut stmt = conn.prepare("SELECT 1 as test")?;
                let rows: Vec<i32> = stmt
                    .query_map([], |row| Ok(row.get(0)?))
                    .and_then(|mapped_rows| mapped_rows.collect())?;
                Ok(rows)
            })
            .unwrap();

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_database_manager_execute() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);
        manager.initialize().unwrap();

        // 创建表
        manager
            .execute("CREATE TABLE test (id INTEGER, name TEXT)")
            .unwrap();

        // 插入数据
        manager
            .execute("INSERT INTO test VALUES (1, 'test')")
            .unwrap();

        // 查询数据
        let result = manager
            .with_connection(|conn| {
                let mut stmt = conn.prepare("SELECT name FROM test WHERE id = 1")?;
                let names: Vec<String> = stmt
                    .query_map([], |row| Ok(row.get(0)?))
                    .and_then(|mapped_rows| mapped_rows.collect())?;
                Ok(names)
            })
            .unwrap();

        assert_eq!(result, vec!["test".to_string()]);
    }

    #[test]
    fn test_database_manager_execute_batch() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);
        manager.initialize().unwrap();

        let batch_sql = r#"
            CREATE TABLE users (id INTEGER, name TEXT);
            INSERT INTO users VALUES (1, 'Alice');
            INSERT INTO users VALUES (2, 'Bob');
        "#;

        manager.execute_batch(batch_sql).unwrap();

        // 验证数据
        let result = manager
            .with_connection(|conn| {
                let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
                let count: i64 = stmt.query_row([], |row| row.get(0))?;
                Ok(count)
            })
            .unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_database_manager_not_initialized() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);

        // 未初始化时应该返回错误
        let result = manager.with_connection(|_conn| Ok(()));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DatabaseError::NotInitialized));
    }

    #[test]
    fn test_database_manager_clone() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager1 = DatabaseManager::new(config);
        manager1.initialize().unwrap();

        // 克隆管理器
        let manager2 = manager1.clone();

        // 两个管理器共享同一个连接
        assert!(manager2.is_initialized());

        // 使用克隆的管理器执行操作
        manager2
            .execute("CREATE TABLE shared (id INTEGER)")
            .unwrap();

        // 原管理器也能看到这个表
        let result = manager1
            .with_connection(|conn| {
                let mut stmt = conn.prepare("SELECT COUNT(*) FROM shared")?;
                let count: i64 = stmt.query_row([], |row| row.get(0))?;
                Ok(count)
            })
            .unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_database_manager_close() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);
        manager.initialize().unwrap();

        assert!(manager.is_initialized());

        manager.close().unwrap();
        assert!(!manager.is_initialized());
    }

    #[test]
    fn test_from_env() {
        std::env::set_var("DATABASE_URL", ":memory:");
        let manager = DatabaseManager::from_env().unwrap();
        manager.initialize().unwrap();

        assert!(manager.is_initialized());
        assert!(manager.config().is_memory);

        std::env::remove_var("DATABASE_URL");
    }
}
