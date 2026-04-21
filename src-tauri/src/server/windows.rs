//! 活动窗口相关 Tauri commands。
//!
//! 窗口查询直接复用 `src-db` 的窗口 API。server 层只负责从 Tauri managed state
//! 取出数据库连接，并把错误转换成前端可接收的字符串。

use crate::server::ServerState;
use database::{
    api::{
        get_observed_window as api_get_observed_window, query_observed_windows,
        ObservedWindowQuery, PaginatedResponse,
    },
    models::ObservedWindowRecord,
};
use tauri::State;

/// 分页查询活动窗口上下文。
///
/// 这个接口用于 `/windows` 页面，支持按应用名、上下文 hash、分页和排序筛选。
#[tauri::command]
pub fn list_observed_windows(
    query: Option<ObservedWindowQuery>,
    state: State<'_, ServerState>,
) -> Result<PaginatedResponse<ObservedWindowRecord>, String> {
    state
        .db()
        .with_connection(|conn| query_observed_windows(conn, &query.unwrap_or_default()))
        .map_err(|error| error.to_string())
}

/// 按窗口主键读取单个活动窗口上下文。
///
/// 这个接口用于 `/windows/:windowId` 详情页；窗口关联事件可继续通过
/// `list_input_events({ window_id })` 查询。
#[tauri::command]
pub fn get_observed_window(
    window_id: i64,
    state: State<'_, ServerState>,
) -> Result<Option<ObservedWindowRecord>, String> {
    state
        .db()
        .with_connection(|conn| api_get_observed_window(conn, window_id))
        .map_err(|error| error.to_string())
}
