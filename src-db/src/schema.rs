//! 数据库表结构定义和初始化

use crate::connection::{DatabaseManager, DatabaseResult};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// 键鼠事件记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub id: Option<i64>,
    pub timestamp: DateTime<Local>,
    pub event_type: String,
    pub event_detail: String,
    pub window_title: Option<String>,
    pub window_app_name: Option<String>,
    pub window_process_path: Option<String>,
    pub window_process_id: Option<u64>,
    pub window_position_x: Option<f64>,
    pub window_position_y: Option<f64>,
    pub window_size_width: Option<f64>,
    pub window_size_height: Option<f64>,
}

/// 窗口信息记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRecord {
    pub id: Option<i64>,
    pub timestamp: DateTime<Local>,
    pub title: String,
    pub app_name: String,
    pub process_path: String,
    pub process_id: u64,
    pub position_x: f64,
    pub position_y: f64,
    pub size_width: f64,
    pub size_height: f64,
    pub is_active: bool,
}

/// 应用使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsageStats {
    pub id: Option<i64>,
    pub app_name: String,
    pub process_path: String,
    pub total_time_seconds: i64,
    pub session_count: i64,
    pub last_used: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

/// 数据库表结构管理
pub struct SchemaManager;

impl SchemaManager {
    /// 初始化所有表结构
    pub fn initialize_tables(db: &DatabaseManager) -> DatabaseResult<()> {
        log::info!("Initializing database tables...");
        
        Self::create_input_events_table(db)?;
        Self::create_window_records_table(db)?;
        Self::create_app_usage_stats_table(db)?;
        Self::create_indexes(db)?;
        
        log::info!("Database tables initialized successfully");
        Ok(())
    }

    /// 创建输入事件表
    fn create_input_events_table(db: &DatabaseManager) -> DatabaseResult<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS input_events (
                id INTEGER PRIMARY KEY,
                timestamp TIMESTAMP NOT NULL,
                event_type VARCHAR(50) NOT NULL,
                event_detail TEXT,
                window_title TEXT,
                window_app_name VARCHAR(255),
                window_process_path TEXT,
                window_process_id BIGINT,
                window_position_x DOUBLE,
                window_position_y DOUBLE,
                window_size_width DOUBLE,
                window_size_height DOUBLE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        "#;
        
        db.execute_batch(sql)?;
        log::debug!("Created input_events table");
        Ok(())
    }

    /// 创建窗口记录表
    fn create_window_records_table(db: &DatabaseManager) -> DatabaseResult<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS window_records (
                id INTEGER PRIMARY KEY,
                timestamp TIMESTAMP NOT NULL,
                title TEXT NOT NULL,
                app_name VARCHAR(255) NOT NULL,
                process_path TEXT NOT NULL,
                process_id BIGINT NOT NULL,
                position_x DOUBLE NOT NULL,
                position_y DOUBLE NOT NULL,
                size_width DOUBLE NOT NULL,
                size_height DOUBLE NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT true,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        "#;
        
        db.execute_batch(sql)?;
        log::debug!("Created window_records table");
        Ok(())
    }

    /// 创建应用使用统计表
    fn create_app_usage_stats_table(db: &DatabaseManager) -> DatabaseResult<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS app_usage_stats (
                id INTEGER PRIMARY KEY,
                app_name VARCHAR(255) NOT NULL,
                process_path TEXT NOT NULL,
                total_time_seconds BIGINT NOT NULL DEFAULT 0,
                session_count BIGINT NOT NULL DEFAULT 0,
                last_used TIMESTAMP NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(app_name, process_path)
            );
        "#;
        
        db.execute_batch(sql)?;
        log::debug!("Created app_usage_stats table");
        Ok(())
    }

    /// 创建索引
    fn create_indexes(db: &DatabaseManager) -> DatabaseResult<()> {
        let indexes = vec![
            // 输入事件表索引
            "CREATE INDEX IF NOT EXISTS idx_input_events_timestamp ON input_events(timestamp);",
            "CREATE INDEX IF NOT EXISTS idx_input_events_type ON input_events(event_type);",
            "CREATE INDEX IF NOT EXISTS idx_input_events_app ON input_events(window_app_name);",
            
            // 窗口记录表索引
            "CREATE INDEX IF NOT EXISTS idx_window_records_timestamp ON window_records(timestamp);",
            "CREATE INDEX IF NOT EXISTS idx_window_records_app ON window_records(app_name);",
            "CREATE INDEX IF NOT EXISTS idx_window_records_process_id ON window_records(process_id);",
            
            // 应用统计表索引
            "CREATE INDEX IF NOT EXISTS idx_app_usage_stats_app_name ON app_usage_stats(app_name);",
            "CREATE INDEX IF NOT EXISTS idx_app_usage_stats_last_used ON app_usage_stats(last_used);",
        ];

        for index_sql in indexes {
            db.execute_batch(index_sql)?;
        }
        
        log::debug!("Created database indexes");
        Ok(())
    }

    /// 检查表是否存在
    pub fn table_exists(db: &DatabaseManager, table_name: &str) -> DatabaseResult<bool> {
        let exists = db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = ?1"
            )?;
            let count: i64 = stmt.query_row([table_name], |row| row.get(0))?;
            Ok(count > 0)
        })?;
        
        Ok(exists)
    }

    /// 获取表的行数
    pub fn get_table_count(db: &DatabaseManager, table_name: &str) -> DatabaseResult<i64> {
        let count = db.with_connection(|conn| {
            let sql = format!("SELECT COUNT(*) FROM {}", table_name);
            let mut stmt = conn.prepare(&sql)?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count)
        })?;
        
        Ok(count)
    }

    /// 清空所有表数据（用于测试）
    #[cfg(test)]
    pub fn clear_all_tables(db: &DatabaseManager) -> DatabaseResult<()> {
        let tables = vec!["input_events", "window_records", "app_usage_stats"];
        
        for table in tables {
            let sql = format!("DELETE FROM {}", table);
            db.execute(&sql)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseConfig, RunMode};

    #[test]
    fn test_schema_initialization() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let db = DatabaseManager::new(config);
        db.initialize().unwrap();
        
        // 初始化表结构
        SchemaManager::initialize_tables(&db).unwrap();
        
        // 检查表是否存在
        assert!(SchemaManager::table_exists(&db, "input_events").unwrap());
        assert!(SchemaManager::table_exists(&db, "window_records").unwrap());
        assert!(SchemaManager::table_exists(&db, "app_usage_stats").unwrap());
        
        // 检查表行数
        assert_eq!(SchemaManager::get_table_count(&db, "input_events").unwrap(), 0);
    }
}
