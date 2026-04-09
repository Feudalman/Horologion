//! TOML 配置文件结构定义模块

use serde::{Deserialize, Serialize};

/// 应用程序完整配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    /// 数据库配置
    #[serde(default)]
    pub database: DatabaseSettings,
    /// 日志配置
    #[serde(default)]
    pub logging: LoggingSettings,
    /// 性能配置
    #[serde(default)]
    pub performance: PerformanceSettings,
    /// 监听器配置
    #[serde(default)]
    pub listener: ListenerSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            database: DatabaseSettings::default(),
            logging: LoggingSettings::default(),
            performance: PerformanceSettings::default(),
            listener: ListenerSettings::default(),
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseSettings {
    /// 数据库文件路径
    pub database_path: Option<String>,
    /// 数据库连接 URL（与 database_path 二选一）
    pub database_url: Option<String>,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            database_path: None,
            database_url: None,
        }
    }
}

/// 日志配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingSettings {
    /// 日志级别：debug, info, warn, error
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

/// 性能配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceSettings {
    /// 缓存大小（页数）
    pub cache_size: Option<i64>,
    /// 内存映射大小（字节）
    pub mmap_size: Option<i64>,
    /// 同步模式：OFF, NORMAL, FULL
    pub synchronous: Option<String>,
    /// 日志模式：DELETE, TRUNCATE, PERSIST, MEMORY, WAL, OFF
    pub journal_mode: Option<String>,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            cache_size: Some(1000000),
            mmap_size: Some(268435456), // 256MB
            synchronous: Some("NORMAL".to_string()),
            journal_mode: Some("WAL".to_string()),
        }
    }
}

/// 监听器配置（预留给未来使用）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListenerSettings {
    /// 是否启用监听器
    pub enabled: Option<bool>,
}

impl Default for ListenerSettings {
    fn default() -> Self {
        Self { enabled: Some(true) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.logging.log_level, "info");
        assert_eq!(settings.performance.cache_size, Some(1000000));
        assert_eq!(settings.performance.synchronous, Some("NORMAL".to_string()));
    }

    #[test]
    fn test_toml_deserialize() {
        let toml_str = r#"
            [database]
            database_path = "/custom/path/db.db"

            [logging]
            log_level = "debug"

            [performance]
            cache_size = 2000000
            synchronous = "FULL"

            [listener]
            enabled = false
        "#;

        let settings: AppSettings = toml::from_str(toml_str).unwrap();
        assert_eq!(
            settings.database.database_path,
            Some("/custom/path/db.db".to_string())
        );
        assert_eq!(settings.logging.log_level, "debug");
        assert_eq!(settings.performance.cache_size, Some(2000000));
        assert_eq!(settings.listener.enabled, Some(false));
    }

    #[test]
    fn test_toml_serialize() {
        let settings = AppSettings {
            database: DatabaseSettings {
                database_path: Some("/test/path.db".to_string()),
                database_url: None,
            },
            logging: LoggingSettings {
                log_level: "warn".to_string(),
            },
            performance: PerformanceSettings::default(),
            listener: ListenerSettings::default(),
        };

        let toml_str = toml::to_string(&settings).unwrap();
        assert!(toml_str.contains("database_path"));
        assert!(toml_str.contains("log_level"));
    }
}
