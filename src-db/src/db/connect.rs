//! # db - connect
//!
//! 该模块负责数据库连接的创建和生命周期管理。

use crate::db::config::DatabaseConfig;
use crate::errors::{DatabaseError, DatabaseResult};
use duckdb::Connection;
use std::sync::{Arc, Mutex};

/// 根据数据库配置打开一个 DuckDB 连接。
///
/// 这个函数只做连接创建，不持有连接状态。调用方如果只需要一次性访问数据库，
/// 可以直接使用它；如果需要在应用生命周期内复用连接，使用 `DatabaseManager`。
pub fn connect(config: &DatabaseConfig) -> DatabaseResult<Connection> {
    config.ensure_parent_dir()?;

    let connection_string = config.connection_string();
    let conn = Connection::open(&connection_string)?;

    log::info!(
        "Database connected: {} (memory: {}, source: {:?})",
        connection_string,
        config.is_memory(),
        config.source
    );

    Ok(conn)
}

/// 从环境变量和默认规则创建配置，并打开数据库连接。
pub fn connect_from_env() -> DatabaseResult<Connection> {
    let config = DatabaseConfig::from_env()?;
    connect(&config)
}

/// 数据库连接管理器。
///
/// `DatabaseManager` 持有数据库配置，并在调用 init 后保存一个可复用连接。
/// 它适合后续提供给 schema、operations、api 等模块使用。
#[derive(Clone)]
pub struct DatabaseManager {
    config: DatabaseConfig,
    connection: Arc<Mutex<Option<Connection>>>,
}

impl DatabaseManager {
    /// 创建一个尚未初始化连接的管理器。
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            config,
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// 根据环境变量和默认规则创建管理器。
    pub fn from_env() -> DatabaseResult<Self> {
        let config = DatabaseConfig::from_env()?;
        Ok(Self::new(config))
    }

    /// 初始化数据库连接。
    ///
    /// 如果已经初始化过，会先关闭旧连接，再用当前配置创建新连接。
    pub fn init(&self) -> DatabaseResult<()> {
        let conn = connect(&self.config)?;

        let mut guard = self.connection.lock().unwrap();
        *guard = Some(conn);

        Ok(())
    }

    /// 使用已初始化的连接执行闭包。
    pub fn with_connection<F, T>(&self, f: F) -> DatabaseResult<T>
    where
        F: FnOnce(&Connection) -> DatabaseResult<T>,
    {
        let guard = self.connection.lock().unwrap();
        let conn = guard.as_ref().ok_or(DatabaseError::NotInitd)?;

        f(conn)
    }

    /// 获取数据库配置。
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// 检查数据库连接是否已经初始化。
    pub fn is_initd(&self) -> bool {
        self.connection.lock().unwrap().is_some()
    }

    /// 关闭当前连接。
    pub fn close(&self) {
        let mut guard = self.connection.lock().unwrap();
        *guard = None;
    }

    /// 执行单条 SQL。
    pub fn execute(&self, sql: &str) -> DatabaseResult<usize> {
        self.with_connection(|conn| Ok(conn.execute(sql, [])?))
    }

    /// 执行一组 SQL。
    pub fn execute_batch(&self, sql: &str) -> DatabaseResult<()> {
        self.with_connection(|conn| Ok(conn.execute_batch(sql)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::path::RunMode;

    #[test]
    fn test_connect_memory() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let conn = connect(&config).unwrap();

        let value: i32 = conn.query_row("SELECT 1", [], |row| row.get(0)).unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn test_database_manager_lifecycle() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);

        assert!(!manager.is_initd());

        manager.init().unwrap();
        assert!(manager.is_initd());

        manager.close();
        assert!(!manager.is_initd());
    }

    #[test]
    fn test_database_manager_execute() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);
        manager.init().unwrap();

        manager
            .execute_batch(
                r#"
                CREATE TABLE test_items (id INTEGER, name TEXT);
                INSERT INTO test_items VALUES (1, 'alpha');
                INSERT INTO test_items VALUES (2, 'beta');
                "#,
            )
            .unwrap();

        let count = manager
            .with_connection(|conn| {
                let count: i64 =
                    conn.query_row("SELECT COUNT(*) FROM test_items", [], |row| row.get(0))?;
                Ok(count)
            })
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_database_manager_not_initd() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let manager = DatabaseManager::new(config);

        let result = manager.with_connection(|_| Ok(()));
        assert!(matches!(result, Err(DatabaseError::NotInitd)));
    }
}
