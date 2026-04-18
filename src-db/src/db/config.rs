//! 数据库配置管理模块
//!
//! 根据不同的运行模式提供数据库路径配置：
//! - 测试模式：内存数据库
//! - 开发模式：项目根目录的 playground/db
//! - 生产模式：系统数据目录或配置文件指定的目录

use crate::db::path::{self, RunMode};
use crate::errors::DatabaseResult;
use std::path::{Path, PathBuf};

/// 数据库配置来源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseSource {
    // 测试模式下使用内存数据库
    TestMemory,
    // 开发模式下使用项目根目录的 playground/db
    Environment,
    // 生产模式下使用系统数据目录或配置文件指定的目录
    ConfigFile(PathBuf),
    // 生产模式下使用默认路径
    DefaultPath,
}

/// 数据库目标
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseTarget {
    // 内存数据库
    Memory,
    // 文件数据库，包含路径
    File(PathBuf),
}

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    // 当前数据库的运行模式（测试、开发、生产）
    pub mode: RunMode,
    // 数据库目标（内存或文件路径）
    pub target: DatabaseTarget,
    // 数据库配置来源（环境变量、配置文件、默认路径等）
    pub source: DatabaseSource,
}

impl DatabaseConfig {
    /// 从环境变量创建配置
    ///
    /// 优先检查环境变量 DATABASE_URL 和 DATABASE_PATH
    /// 如果未设置，则根据运行模式使用默认路径或内存数据库
    ///
    /// 建议使用远程数据库时，设置 DATABASE_URL 环境变量；本地时，可以设置 DATABASE_PATH 或使用默认路径。
    pub fn from_env() -> DatabaseResult<Self> {
        if let Ok(value) = std::env::var("DATABASE_URL") {
            return Self::from_database_value(
                RunMode::from_env(),
                value,
                DatabaseSource::Environment,
            );
        }

        if let Ok(value) = std::env::var("DATABASE_PATH") {
            return Self::from_database_value(
                RunMode::from_env(),
                value,
                DatabaseSource::Environment,
            );
        }

        Self::new(RunMode::from_env())
    }

    /// 根据运行模式创建数据库配置
    ///
    ///  Test 模式下使用内存数据库
    ///  Development 模式下使用项目根目录的 playground/db/horologion.db 作为默认路径
    ///  Production 模式下优先检查配置文件指定的路径，如果没有配置文件或未配置数据库路径，则使用默认路径
    pub fn new(mode: RunMode) -> DatabaseResult<Self> {
        match mode {
            RunMode::Test => Ok(Self {
                mode,
                target: DatabaseTarget::Memory,
                source: DatabaseSource::TestMemory,
            }),

            RunMode::Development => {
                // 开发模式下一般使用项目根目录的 playground/db/horologion.db 作为默认路径
                let db_path = path::get_default_db_path(&mode)?;
                Ok(Self {
                    mode,
                    target: DatabaseTarget::File(db_path),
                    source: DatabaseSource::DefaultPath,
                })
            }

            RunMode::Production => {
                // 生产模式下优先检查配置文件指定的路径
                if let Some(config_path) = path::get_config_file_path(&mode) {
                    // 如果能够找到配置文件，尝试读取数据库路径
                    if let Some(db_path) = Self::read_db_path_from_config_file(&config_path)? {
                        return Ok(Self {
                            mode,
                            target: DatabaseTarget::File(db_path),
                            source: DatabaseSource::ConfigFile(config_path),
                        });
                    }
                }

                // 如果没有配置文件，或者未配置数据库路径，使用默认路径
                let db_path = path::get_default_db_path(&mode)?;
                Ok(Self {
                    mode,
                    target: DatabaseTarget::File(db_path),
                    source: DatabaseSource::DefaultPath,
                })
            }
        }
    }

    /// 检查是否使用内存数据库
    pub fn is_memory(&self) -> bool {
        matches!(self.target, DatabaseTarget::Memory)
    }

    /// 当前数据库的路径
    pub fn db_path(&self) -> Option<&Path> {
        match &self.target {
            DatabaseTarget::Memory => None,
            DatabaseTarget::File(path) => Some(path),
        }
    }

    /// 数据库连接字符串
    pub fn connection_string(&self) -> String {
        match &self.target {
            DatabaseTarget::Memory => ":memory:".to_string(),
            DatabaseTarget::File(path) => path.to_string_lossy().to_string(),
        }
    }

    /// 确保数据库文件所在目录存在
    ///
    /// 如果是内存数据库，则不需要创建目录。
    /// 如果是文件数据库，则确保父目录存在，如果不存在则创建。
    pub fn ensure_parent_dir(&self) -> DatabaseResult<()> {
        // 只有当数据库目标是文件路径时，才需要确保目录存在
        if let Some(db_path) = self.db_path() {
            if let Some(parent) = db_path.parent() {
                path::ensure_directory_exists(parent)?;
            }
        }

        Ok(())
    }

    /// 从 value 创建一个新的 `DatabaseConfig`
    ///
    /// 如果 value 是 ":memory:"，则创建一个内存数据库配置。
    /// 如果 value 是一个路径字符串，则创建一个文件数据库配置，路径为该字符串。
    ///
    /// # Arguments
    ///
    /// * `mode`: 当前运行模式。
    /// * `value`: 数据库值字符串。
    /// * `source`: 数据库值字符串的来源。
    ///
    /// # Returns
    ///
    /// 返回一个 `DatabaseConfig` 实例，表示根据提供的值创建的数据库配置。
    fn from_database_value(
        mode: RunMode,
        value: String,
        source: DatabaseSource,
    ) -> DatabaseResult<Self> {
        if value == ":memory:" {
            return Ok(Self {
                mode,
                target: DatabaseTarget::Memory,
                source,
            });
        }

        Ok(Self {
            mode,
            target: DatabaseTarget::File(PathBuf::from(value)),
            source,
        })
    }

    /// 读取配置文件指定的数据库路径
    ///
    /// 如果配置文件中指定了数据库路径，则返回该路径。
    /// 否则返回 None。
    fn read_db_path_from_config_file(path: &Path) -> DatabaseResult<Option<PathBuf>> {
        // 这里可以先做最小实现，后面再扩展成完整 AppSettings。
        #[derive(serde::Deserialize)]
        struct FileConfig {
            database: Option<DatabaseSection>,
        }

        #[derive(serde::Deserialize)]
        struct DatabaseSection {
            database_url: Option<String>,
            database_path: Option<String>,
        }

        let content = std::fs::read_to_string(path)?;
        let config: FileConfig = toml::from_str(&content)?;

        let value = config
            .database
            .and_then(|database| database.database_url.or(database.database_path));

        Ok(value.map(PathBuf::from))
    }
}
