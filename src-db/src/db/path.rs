//! 路径解析模块
//!
//! 根据不同的运行模式和操作系统提供正确的数据库路径和配置文件路径

use crate::errors::{DatabaseError, DatabaseResult};
use config::{app, database, env, paths};
use std::path::{Path, PathBuf};

/// 运行模式枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunMode {
    /// 测试模式 - 使用内存数据库
    Test,
    /// 开发模式 - 使用项目本地目录
    Development,
    /// 生产模式 - 使用系统数据目录或配置文件指定的目录
    Production,
}

impl RunMode {
    /// 从环境变量获取运行模式
    ///
    /// 检查 RUN_MODE 环境变量，如果未设置则根据构建模式推断：
    /// - Debug 构建默认为 Development
    /// - Release 构建默认为 Production
    pub fn from_env() -> Self {
        match std::env::var(env::RUN_MODE)
            .ok()
            .as_deref()
            .map(|s| s.to_lowercase())
            .as_deref()
        {
            Some("test") => Self::Test,
            Some("dev") | Some("development") => Self::Development,
            Some("prod") | Some("production") => Self::Production,
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

/// 获取配置文件路径
///
/// 只在生产模式下返回配置文件路径，优先检查用户目录，然后检查系统目录
pub fn get_config_file_path(mode: &RunMode) -> Option<PathBuf> {
    match mode {
        RunMode::Test | RunMode::Development => None,
        RunMode::Production => {
            // 尝试多个可能的配置文件位置
            let candidates = get_config_path_candidates();

            // 返回第一个存在的配置文件
            for path in candidates {
                if path.exists() {
                    log::info!("Found config file at: {:?}", path);
                    return Some(path);
                }
            }

            log::debug!("No config file found in standard locations");
            None
        }
    }
}

/// 获取配置文件候选路径列表（按优先级排序）
fn get_config_path_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // 用户配置目录
    if let Some(config_dir) = dirs::config_dir() {
        candidates.push(config_dir.join(app::NAME).join(paths::CONFIG_FILE_NAME));
    }

    // 系统配置目录（平台相关）
    #[cfg(target_os = "linux")]
    {
        candidates.push(PathBuf::from(format!(
            "/etc/config/{}/{}",
            app::NAME,
            paths::CONFIG_FILE_NAME
        )));
        candidates.push(PathBuf::from(format!(
            "/etc/{}/{}",
            app::NAME,
            paths::CONFIG_FILE_NAME
        )));
    }

    #[cfg(target_os = "macos")]
    {
        candidates.push(PathBuf::from(format!(
            "/Library/Preferences/{}/{}",
            app::NAME,
            paths::CONFIG_FILE_NAME
        )));
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(program_data) = std::env::var_os("PROGRAMDATA") {
            let path = PathBuf::from(program_data)
                .join(app::NAME)
                .join(paths::CONFIG_FILE_NAME);
            candidates.push(path);
        }
    }

    candidates
}

/// 获取默认数据库路径
///
/// 根据运行模式返回默认的数据库文件路径
/// - Test 直接报错，因为不应该使用文件数据库，需要在调用前判断
/// - Development 返回项目本地数据库路径
/// - Production 返回系统数据目录或配置文件指定的目录
pub fn get_default_db_path(mode: &RunMode) -> DatabaseResult<PathBuf> {
    match mode {
        RunMode::Test => Err(DatabaseError::InvalidConfig(
            "Test mode should use :memory:, not a file path".to_string(),
        )),
        RunMode::Development => {
            let project_root = find_project_root()?;
            let db_dir = project_root
                .join(paths::PLAYGROUND_DIR)
                .join(paths::DATABASE_DIR);
            ensure_directory_exists(&db_dir)?;
            Ok(db_dir.join(database::FILE_NAME))
        }
        RunMode::Production => {
            let data_dir = dirs::data_dir().ok_or_else(|| {
                DatabaseError::PathNotFound("Failed to get system data directory".to_string())
            })?;
            let app_data_dir = data_dir.join(app::NAME);
            ensure_directory_exists(&app_data_dir)?;
            Ok(app_data_dir.join(database::FILE_NAME))
        }
    }
}

/// 查找项目根目录
///
/// 从当前目录开始向上查找，寻找包含 Cargo.toml 的目录。如果找到 workspace 根目录则返回，否则返回最近的包目录
pub fn find_project_root() -> DatabaseResult<PathBuf> {
    let current_dir = std::env::current_dir().map_err(|e| {
        DatabaseError::PathNotFound(format!("Failed to get current directory: {}", e))
    })?;

    find_project_root_from(&current_dir)
}

/// 查找项目根目录
///
/// 从指定目录开始向上查找，寻找包含 Cargo.toml 的目录。如果找到 workspace 根目录则返回，否则返回最近的包目录
fn find_project_root_from(start_dir: &Path) -> DatabaseResult<PathBuf> {
    let mut path = start_dir;
    let mut nearest_manifest_dir = None;

    loop {
        let manifest_path = path.join("Cargo.toml");
        if manifest_path.exists() {
            nearest_manifest_dir.get_or_insert_with(|| path.to_path_buf());

            if is_workspace_manifest(&manifest_path)? {
                log::debug!("Found workspace root at: {:?}", path);
                return Ok(path.to_path_buf());
            }
        }

        match path.parent() {
            Some(parent) => path = parent,
            None => break,
        }
    }

    match nearest_manifest_dir {
        Some(path) => {
            log::debug!("Found package root at: {:?}", path);
            return Ok(path.to_path_buf());
        }
        None => {
            log::warn!("Could not find project root with Cargo.toml, using current directory");
            Ok(start_dir.to_path_buf())
        }
    }
}

/// 检查 Cargo.toml 是否为 workspace 根目录
fn is_workspace_manifest(manifest_path: &Path) -> DatabaseResult<bool> {
    let content = std::fs::read_to_string(manifest_path)?;
    let manifest = content.parse::<toml::Value>()?;
    Ok(manifest
        .get("workspace")
        .and_then(toml::Value::as_table)
        .is_some())
}

/// 确保目录存在，如果不存在则创建
pub fn ensure_directory_exists(path: &Path) -> DatabaseResult<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        log::info!("Created directory: {:?}", path);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试不存在环境变量时的运行模式
    #[test]
    fn test_run_mode_from_env() {
        std::env::remove_var(env::RUN_MODE);
        let mode = RunMode::from_env();
        let expected = if cfg!(debug_assertions) {
            RunMode::Development
        } else {
            RunMode::Production
        };
        assert_eq!(mode, expected);
    }

    #[test]
    fn test_run_mode_variants() {
        std::env::set_var(env::RUN_MODE, "test");
        assert_eq!(RunMode::from_env(), RunMode::Test);

        std::env::set_var(env::RUN_MODE, "dev");
        assert_eq!(RunMode::from_env(), RunMode::Development);

        std::env::set_var(env::RUN_MODE, "development");
        assert_eq!(RunMode::from_env(), RunMode::Development);

        std::env::set_var(env::RUN_MODE, "prod");
        assert_eq!(RunMode::from_env(), RunMode::Production);

        std::env::set_var(env::RUN_MODE, "production");
        assert_eq!(RunMode::from_env(), RunMode::Production);

        // 清理
        std::env::remove_var(env::RUN_MODE);
    }

    #[test]
    fn test_config_file_path_test_mode() {
        let path = get_config_file_path(&RunMode::Test);
        assert!(path.is_none());
    }

    #[test]
    fn test_config_file_path_dev_mode() {
        let path = get_config_file_path(&RunMode::Development);
        assert!(path.is_none());
    }

    #[test]
    fn test_default_db_path_test_mode() {
        let result = get_default_db_path(&RunMode::Test);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_db_path_development() {
        let result = get_default_db_path(&RunMode::Development);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(paths::PLAYGROUND_DIR));
        assert!(path.to_string_lossy().contains(database::FILE_NAME));
    }

    #[test]
    fn test_find_project_root() {
        // 应该能找到项目根目录（因为我们在项目内运行测试）
        let result = find_project_root();
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(root.join("Cargo.toml").exists());
    }

    #[test]
    fn test_find_project_root_prefers_workspace_root() {
        use std::fs;

        let temp_dir =
            std::env::temp_dir().join(format!("horologion_workspace_test_{}", std::process::id()));
        let workspace_dir = temp_dir.join("workspace");
        let member_dir = workspace_dir.join("crates").join("database");

        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&member_dir).unwrap();
        fs::write(
            workspace_dir.join("Cargo.toml"),
            r#"
[workspace]
members = ["crates/database"]
"#,
        )
        .unwrap();
        fs::write(
            member_dir.join("Cargo.toml"),
            r#"
[package]
name = "database"
version = "0.0.1"
edition = "2021"
"#,
        )
        .unwrap();

        let root = find_project_root_from(&member_dir).unwrap();
        assert_eq!(root, workspace_dir);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ensure_directory_exists() {
        use std::fs;

        let temp_dir = std::env::temp_dir().join("horologion_test");
        let test_dir = temp_dir.join("test_create_dir");

        // 清理可能存在的目录
        let _ = fs::remove_dir_all(&test_dir);

        // 测试创建目录
        let result = ensure_directory_exists(&test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());

        // 测试已存在的目录
        let result = ensure_directory_exists(&test_dir);
        assert!(result.is_ok());

        // 清理
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
