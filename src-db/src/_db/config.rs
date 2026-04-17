//! 数据库配置管理模块
//!
//! 根据不同的运行模式提供数据库路径配置：
//! - 测试模式：内存数据库
//! - 开发模式：项目根目录的 playground/db
//! - 生产模式：系统数据目录或配置文件指定的目录

use crate::db::error::{DatabaseError, DatabaseResult};
use crate::db::paths::{self, RunMode};
use crate::db::settings::{AppSettings, PerformanceSettings};
use std::path::PathBuf;

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// 数据库连接字符串
    pub connection_string: String,
    /// 是否为内存数据库
    pub is_memory: bool,
    /// 数据库文件路径（如果不是内存数据库）
    pub db_path: Option<PathBuf>,
    /// 性能配置
    pub performance: PerformanceSettings,
}

impl DatabaseConfig {
    /// 根据运行模式创建数据库配置
    pub fn new(mode: RunMode) -> DatabaseResult<Self> {
        match mode {
            RunMode::Test => Ok(Self {
                connection_string: ":memory:".to_string(),
                is_memory: true,
                db_path: None,
                performance: PerformanceSettings::default(),
            }),
            RunMode::Development | RunMode::Production => {
                // 尝试从配置文件读取
                if let Some(config_path) = paths::get_config_file_path(&mode) {
                    if let Ok(config) = Self::from_config_file(&config_path) {
                        log::info!("Using database config from file: {:?}", config_path);
                        return Ok(config);
                    }
                }

                // 如果没有配置文件，使用默认路径
                let db_path = paths::get_default_db_path(&mode)?;

                Ok(Self {
                    connection_string: db_path.to_string_lossy().to_string(),
                    is_memory: false,
                    db_path: Some(db_path),
                    performance: PerformanceSettings::default(),
                })
            }
        }
    }

    /// 从环境变量创建配置
    ///
    /// 配置优先级：
    /// 1. DATABASE_URL 或 DATABASE_PATH 环境变量
    /// 2. TOML 配置文件
    /// 3. 默认路径（基于运行模式）
    pub fn from_env() -> DatabaseResult<Self> {
        // 优先使用环境变量中的数据库路径
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            return Self::from_connection_string(&db_url);
        }

        if let Ok(db_path) = std::env::var("DATABASE_PATH") {
            return Self::from_connection_string(&db_path);
        }

        // 否则根据运行模式使用默认配置
        let mode = RunMode::from_env();
        Self::new(mode)
    }

    /// 从连接字符串创建配置
    fn from_connection_string(connection_string: &str) -> DatabaseResult<Self> {
        let is_memory = connection_string == ":memory:";
        let db_path = if is_memory {
            None
        } else {
            Some(PathBuf::from(connection_string))
        };

        Ok(Self {
            connection_string: connection_string.to_string(),
            is_memory,
            db_path,
            performance: PerformanceSettings::default(),
        })
    }

    /// 从 TOML 配置文件读取配置
    pub fn from_config_file(path: &std::path::Path) -> DatabaseResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            DatabaseError::Config(format!("Failed to read config file {:?}: {}", path, e))
        })?;

        let settings: AppSettings = toml::from_str(&content)?;

        // 从配置文件获取数据库路径
        let connection_string = settings
            .database
            .database_url
            .or(settings.database.database_path)
            .ok_or_else(|| {
                DatabaseError::InvalidConfig(
                    "Config file must specify database_url or database_path".to_string(),
                )
            })?;

        let is_memory = connection_string == ":memory:";
        let db_path = if is_memory {
            None
        } else {
            Some(PathBuf::from(&connection_string))
        };

        Ok(Self {
            connection_string,
            is_memory,
            db_path,
            performance: settings.performance,
        })
    }

    /// 设置性能参数（Builder 模式）
    pub fn with_performance(mut self, performance: PerformanceSettings) -> Self {
        self.performance = performance;
        self
    }

    /// 确保数据库目录存在
    pub fn ensure_db_directory(&self) -> DatabaseResult<()> {
        if let Some(db_path) = &self.db_path {
            if let Some(parent) = db_path.parent() {
                paths::ensure_directory_exists(parent)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_memory() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        assert!(config.is_memory);
        assert_eq!(config.connection_string, ":memory:");
        assert!(config.db_path.is_none());
    }

    #[test]
    fn test_database_config_development() {
        let config = DatabaseConfig::new(RunMode::Development).unwrap();
        assert!(!config.is_memory);
        assert!(config.db_path.is_some());
        assert!(config.connection_string.contains("playground"));
        assert!(config.connection_string.contains("horologion.db"));
    }

    #[test]
    fn test_from_connection_string_memory() {
        let config = DatabaseConfig::from_connection_string(":memory:").unwrap();
        assert!(config.is_memory);
        assert_eq!(config.connection_string, ":memory:");
        assert!(config.db_path.is_none());
    }

    #[test]
    fn test_from_connection_string_file() {
        let config = DatabaseConfig::from_connection_string("/tmp/test.db").unwrap();
        assert!(!config.is_memory);
        assert_eq!(config.connection_string, "/tmp/test.db");
        assert_eq!(config.db_path, Some(PathBuf::from("/tmp/test.db")));
    }

    #[test]
    fn test_from_env_with_database_url() {
        std::env::set_var("DATABASE_URL", ":memory:");
        let config = DatabaseConfig::from_env().unwrap();
        assert!(config.is_memory);
        std::env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_from_env_with_database_path() {
        std::env::set_var("DATABASE_PATH", "/custom/db.db");
        let config = DatabaseConfig::from_env().unwrap();
        assert!(!config.is_memory);
        assert_eq!(config.connection_string, "/custom/db.db");
        std::env::remove_var("DATABASE_PATH");
    }

    #[test]
    fn test_with_performance() {
        let mut perf = PerformanceSettings::default();
        perf.cache_size = Some(5000000);

        let config = DatabaseConfig::new(RunMode::Test)
            .unwrap()
            .with_performance(perf.clone());

        assert_eq!(config.performance.cache_size, Some(5000000));
    }

    #[test]
    fn test_from_config_file_toml() {
        use std::fs;
        use std::io::Write;

        // 创建临时配置文件
        let temp_dir = std::env::temp_dir().join("horologion_config_test");
        fs::create_dir_all(&temp_dir).unwrap();
        let config_path = temp_dir.join("test_config.toml");

        let toml_content = r#"
[database]
database_path = "/test/custom/db.db"

[logging]
log_level = "debug"

[performance]
cache_size = 2000000
synchronous = "FULL"
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        // 测试读取配置
        let config = DatabaseConfig::from_config_file(&config_path).unwrap();
        assert_eq!(config.connection_string, "/test/custom/db.db");
        assert!(!config.is_memory);
        assert_eq!(config.performance.cache_size, Some(2000000));
        assert_eq!(config.performance.synchronous, Some("FULL".to_string()));

        // 清理
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_from_config_file_missing_db_path() {
        use std::fs;
        use std::io::Write;

        let temp_dir = std::env::temp_dir().join("horologion_config_test2");
        fs::create_dir_all(&temp_dir).unwrap();
        let config_path = temp_dir.join("test_config.toml");

        let toml_content = r#"
[logging]
log_level = "debug"
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        // 应该返回错误，因为没有指定数据库路径
        let result = DatabaseConfig::from_config_file(&config_path);
        assert!(result.is_err());

        // 清理
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
