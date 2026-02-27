//! 数据库配置管理模块
//! 
//! 根据不同的运行模式提供数据库路径配置：
//! - 测试模式：内存数据库
//! - 开发模式：项目根目录的 playground/db
//! - 生产模式：系统数据目录下的 horologion

use std::path::PathBuf;

/// 运行模式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum RunMode {
    /// 测试模式 - 使用内存数据库
    Test,
    /// 开发模式 - 使用项目本地目录
    Development,
    /// 生产模式 - 使用系统数据目录
    Production,
}

impl RunMode {
    /// 从环境变量获取运行模式
    pub fn from_env() -> Self {
        match std::env::var("RUN_MODE").as_deref() {
            Ok("test") => Self::Test,
            Ok("dev") | Ok("development") => Self::Development,
            Ok("prod") | Ok("production") => Self::Production,
            _ => {
                // 默认根据是否为 debug 构建来判断
                if cfg!(debug_assertions) {
                    Self::Development
                } else {
                    Self::Production
                }
            }
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// 数据库连接字符串
    pub connection_string: String,
    /// 是否为内存数据库
    pub is_memory: bool,
    /// 数据库文件路径（如果不是内存数据库）
    pub db_path: Option<PathBuf>,
}

impl DatabaseConfig {
    /// 根据运行模式创建数据库配置
    pub fn new(mode: RunMode) -> Result<Self, Box<dyn std::error::Error>> {
        match mode {
            RunMode::Test => Ok(Self {
                connection_string: ":memory:".to_string(),
                is_memory: true,
                db_path: None,
            }),
            RunMode::Development => {
                let db_dir = Self::get_development_db_path()?;
                let db_file = db_dir.join("horologion.db");
                
                Ok(Self {
                    connection_string: db_file.to_string_lossy().to_string(),
                    is_memory: false,
                    db_path: Some(db_file),
                })
            }
            RunMode::Production => {
                let db_dir = Self::get_production_db_path()?;
                let db_file = db_dir.join("horologion.db");
                
                Ok(Self {
                    connection_string: db_file.to_string_lossy().to_string(),
                    is_memory: false,
                    db_path: Some(db_file),
                })
            }
        }
    }

    /// 从环境变量创建配置，如果没有环境变量则使用默认配置
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // 优先使用环境变量中的数据库路径
        if let Ok(db_path) = std::env::var("DATABASE_URL") {
            return Ok(Self {
                connection_string: db_path.clone(),
                is_memory: db_path == ":memory:",
                db_path: if db_path == ":memory:" {
                    None
                } else {
                    Some(PathBuf::from(db_path))
                },
            });
        }

        // 否则根据运行模式使用默认配置
        let mode = RunMode::from_env();
        Self::new(mode)
    }

    /// 获取开发环境数据库路径
    fn get_development_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        // 尝试找到项目根目录
        let current_dir = std::env::current_dir()?;
        let mut project_root = current_dir.as_path();
        
        // 向上查找包含 Cargo.toml 的目录作为项目根目录
        loop {
            if project_root.join("Cargo.toml").exists() {
                break;
            }
            if let Some(parent) = project_root.parent() {
                project_root = parent;
            } else {
                // 如果找不到项目根目录，使用当前目录
                project_root = current_dir.as_path();
                break;
            }
        }
        
        let db_dir = project_root.join("playground").join("db");
        
        // 确保目录存在
        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)?;
            log::info!("Created development database directory: {:?}", db_dir);
        }
        
        Ok(db_dir)
    }

    /// 获取生产环境数据库路径
    fn get_production_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .ok_or("Failed to get system data directory")?;
        
        let app_data_dir = data_dir.join("horologion");
        
        // 确保目录存在
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)?;
            log::info!("Created production database directory: {:?}", app_data_dir);
        }
        
        Ok(app_data_dir)
    }

    /// 确保数据库目录存在
    pub fn ensure_db_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(db_path) = &self.db_path {
            if let Some(parent) = db_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                    log::info!("Created database directory: {:?}", parent);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_mode_from_env() {
        // 测试默认模式
        std::env::remove_var("RUN_MODE");
        let mode = RunMode::from_env();
        assert_eq!(mode, if cfg!(debug_assertions) { RunMode::Development } else { RunMode::Production });

        // 测试各种环境变量值
        std::env::set_var("RUN_MODE", "test");
        assert_eq!(RunMode::from_env(), RunMode::Test);

        std::env::set_var("RUN_MODE", "dev");
        assert_eq!(RunMode::from_env(), RunMode::Development);

        std::env::set_var("RUN_MODE", "production");
        assert_eq!(RunMode::from_env(), RunMode::Production);
    }

    #[test]
    fn test_database_config_memory() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        assert!(config.is_memory);
        assert_eq!(config.connection_string, ":memory:");
        assert!(config.db_path.is_none());
    }
}
