//! 运行状态和设置相关 Tauri commands。
//!
//! 这里提供前端 Settings/Overview 页面会用到的应用状态和只读设置。

use crate::server::ServerState;
use chrono::{DateTime, Utc};
use database::db::RunMode;
use duckdb::OptionalExt;
use serde::Serialize;
use tauri::State;

/// 前端可展示的运行模式。
///
/// `src-db` 内部的 `RunMode` 不直接暴露给前端，这里通过轻量 DTO 控制序列化格式。
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunModeResponse {
    Test,
    Development,
    Production,
}

impl From<&RunMode> for RunModeResponse {
    fn from(mode: &RunMode) -> Self {
        match mode {
            RunMode::Test => Self::Test,
            RunMode::Development => Self::Development,
            RunMode::Production => Self::Production,
        }
    }
}

/// 应用当前运行状态。
#[derive(Debug, Clone, Serialize)]
pub struct AppStatus {
    /// listener sidecar 是否已启动且未收到退出事件。
    pub listener_running: bool,
    /// 数据库连接是否可用。
    pub database_ready: bool,
    /// 最近一条输入事件的发生时间；无事件时为 `None`。
    pub last_event_at: Option<DateTime<Utc>>,
    /// 当前数据库运行模式。
    pub run_mode: RunModeResponse,
    /// Tauri 主程序 Cargo 包版本。
    pub app_version: String,
    /// 当前数据库文件路径；内存数据库时为 `None`。
    pub database_path: Option<String>,
}

/// Settings 页面需要的只读应用设置。
#[derive(Debug, Clone, Serialize)]
pub struct AppSettings {
    /// 当前数据库运行模式。
    pub run_mode: RunModeResponse,
    /// Tauri 主程序 Cargo 包版本。
    pub app_version: String,
    /// 当前数据库文件路径；内存数据库时为 `None`。
    pub database_path: Option<String>,
}

/// 返回前端展示所需的应用状态。
#[tauri::command]
pub fn get_app_status(state: State<'_, ServerState>) -> Result<AppStatus, String> {
    // database_ready 只做轻量连通性检查，不在状态接口里触发复杂查询。
    let database_ready = state
        .db()
        .with_connection(|conn| {
            let _: i64 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
            Ok(())
        })
        .is_ok();

    // 最近事件用于 Overview 判断采集是否确实产生了数据。
    let last_event_at = state
        .db()
        .with_connection(|conn| {
            conn.query_row(
                "SELECT occurred_at FROM input_events ORDER BY occurred_at DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(Into::into)
        })
        .map_err(|error| error.to_string())?;

    Ok(AppStatus {
        listener_running: state.is_listener_running(),
        database_ready,
        last_event_at,
        run_mode: RunModeResponse::from(state.run_mode()),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        database_path: state.database_path(),
    })
}

/// 返回 Settings 页面展示所需的只读应用设置。
#[tauri::command]
pub fn get_app_settings(state: State<'_, ServerState>) -> Result<AppSettings, String> {
    Ok(AppSettings {
        run_mode: RunModeResponse::from(state.run_mode()),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        database_path: state.database_path(),
    })
}
