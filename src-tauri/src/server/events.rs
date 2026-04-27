//! 输入事件相关 Tauri commands。
//!
//! 这里负责把前端请求转换为 `src-db` API 调用，并补充 Overview 页面需要的聚合统计。
//! 事件明细查询直接复用 `database::api::query_input_events`，聚合统计暂时在 server 层
//! 通过 SQL 完成，后续如果多个端复用再下沉到 `src-db`。

use crate::server::ServerState;
use chrono::{DateTime, Utc};
use config::overview;
use database::{
    api::{
        get_input_event as api_get_input_event, query_input_events, InputEventQuery,
        InputEventWithWindow, PaginatedResponse,
    },
    models::InputEventRecord,
};
use duckdb::{params_from_iter, Connection, ToSql};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ActivitySummaryQuery {
    /// 统计开始时间，未传则不限制开始时间。
    pub start_at: Option<DateTime<Utc>>,
    /// 统计结束时间，未传则不限制结束时间。
    pub end_at: Option<DateTime<Utc>>,
}

/// Overview 页面需要的活动聚合数据。
#[derive(Debug, Clone, Serialize)]
pub struct ActivitySummary {
    /// 当前查询条件下的总事件数。
    pub total_events: i64,
    /// 键盘按下和释放事件总数。
    pub key_events: i64,
    /// 鼠标按钮按下和释放事件总数。
    pub button_events: i64,
    /// 滚轮事件总数。
    pub wheel_events: i64,
    /// 当前查询条件下出现过事件的窗口数量。
    pub active_windows: i64,
    /// 按事件数量排序的应用 Top 列表。
    pub top_apps: Vec<TopAppSummary>,
}

/// 单个应用在活动统计中的占比。
#[derive(Debug, Clone, Serialize)]
pub struct TopAppSummary {
    /// 活动窗口所属应用名。
    pub app_name: String,
    /// 当前应用关联的事件数量。
    pub event_count: i64,
    /// 当前应用事件数占总事件数的百分比。
    pub share: f64,
}

/// 返回 Overview 页面所需的事件聚合统计。
#[tauri::command]
pub fn get_activity_summary(
    query: Option<ActivitySummaryQuery>,
    state: State<'_, ServerState>,
) -> Result<ActivitySummary, String> {
    let query = query.unwrap_or_default();

    state
        .db()
        .with_connection(|conn| {
            let total_events = count_events(conn, &query, None)?;
            let key_events = count_events(
                conn,
                &query,
                Some("e.event_kind IN ('key_press', 'key_release')"),
            )?;
            let button_events = count_events(
                conn,
                &query,
                Some("e.event_kind IN ('button_press', 'button_release')"),
            )?;
            let wheel_events = count_events(conn, &query, Some("e.event_kind = 'wheel'"))?;
            let active_windows = count_active_windows(conn, &query)?;
            let top_apps = query_top_apps(conn, &query, total_events)?;

            Ok(ActivitySummary {
                total_events,
                key_events,
                button_events,
                wheel_events,
                active_windows,
                top_apps,
            })
        })
        .map_err(|error| error.to_string())
}

/// 分页查询输入事件，并附带事件发生时的窗口上下文。
#[tauri::command]
pub fn list_input_events(
    query: Option<InputEventQuery>,
    state: State<'_, ServerState>,
) -> Result<PaginatedResponse<InputEventWithWindow>, String> {
    state
        .db()
        .with_connection(|conn| query_input_events(conn, &query.unwrap_or_default()))
        .map_err(|error| error.to_string())
}

/// 按事件主键读取单条输入事件。
///
/// 这个接口用于事件详情抽屉或调试视图，返回的记录包含 `raw_event`、`raw_window`
/// 以及 collector 信息。
#[tauri::command]
pub fn get_input_event(
    event_id: i64,
    state: State<'_, ServerState>,
) -> Result<Option<InputEventRecord>, String> {
    state
        .db()
        .with_connection(|conn| api_get_input_event(conn, event_id))
        .map_err(|error| error.to_string())
}

/// 按可选时间范围和额外事件条件统计事件数。
fn count_events(
    conn: &Connection,
    query: &ActivitySummaryQuery,
    extra_condition: Option<&'static str>,
) -> database::errors::DatabaseResult<i64> {
    let (where_sql, values) = build_where_clause(query, extra_condition);
    let sql = format!("SELECT COUNT(*) FROM input_events e{where_sql}");

    Ok(conn.query_row(
        &sql,
        params_from_iter(values.iter().map(|value| value.as_ref())),
        |row| row.get(0),
    )?)
}

/// 统计有窗口关联的去重窗口数量。
fn count_active_windows(
    conn: &Connection,
    query: &ActivitySummaryQuery,
) -> database::errors::DatabaseResult<i64> {
    let (where_sql, values) = build_where_clause(query, Some("e.window_id IS NOT NULL"));
    let sql = format!("SELECT COUNT(DISTINCT e.window_id) FROM input_events e{where_sql}");

    Ok(conn.query_row(
        &sql,
        params_from_iter(values.iter().map(|value| value.as_ref())),
        |row| row.get(0),
    )?)
}

/// 查询事件数量最高的应用列表。
fn query_top_apps(
    conn: &Connection,
    query: &ActivitySummaryQuery,
    total_events: i64,
) -> database::errors::DatabaseResult<Vec<TopAppSummary>> {
    let (where_sql, values) = build_where_clause(query, None);
    let sql = format!(
        r#"
        SELECT
            COALESCE(w.app_name, 'Unknown') AS app_name,
            COUNT(*) AS event_count
        FROM input_events e
        LEFT JOIN observed_windows w ON e.window_id = w.window_id
        {where_sql}
        GROUP BY COALESCE(w.app_name, 'Unknown')
        ORDER BY event_count DESC, app_name ASC
        LIMIT {}
        "#,
        overview::TOP_APPS_LIMIT
    );

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(
        params_from_iter(values.iter().map(|value| value.as_ref())),
        |row| {
            let event_count: i64 = row.get(1)?;
            // share 由 server 层计算，避免不同 SQL 方言中浮点除法细节影响返回值。
            let share = if total_events == 0 {
                0.0
            } else {
                (event_count as f64 / total_events as f64) * 100.0
            };

            Ok(TopAppSummary {
                app_name: row.get(0)?,
                event_count,
                share,
            })
        },
    )?;

    let mut apps = Vec::new();
    for row in rows {
        apps.push(row?);
    }

    Ok(apps)
}

/// 构建聚合查询的 WHERE 子句和参数。
///
/// `extra_condition` 只接受本模块内写死的 SQL 片段，用户输入始终通过参数绑定传入。
fn build_where_clause(
    query: &ActivitySummaryQuery,
    extra_condition: Option<&'static str>,
) -> (String, Vec<Box<dyn ToSql>>) {
    let mut conditions = Vec::new();
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(start_at) = query.start_at {
        conditions.push("e.occurred_at >= ?");
        values.push(Box::new(start_at));
    }

    if let Some(end_at) = query.end_at {
        conditions.push("e.occurred_at <= ?");
        values.push(Box::new(end_at));
    }

    if let Some(condition) = extra_condition {
        conditions.push(condition);
    }

    if conditions.is_empty() {
        return (String::new(), values);
    }

    (format!(" WHERE {}", conditions.join(" AND ")), values)
}
