//! 活动窗口上下文 API。
//!
//! 这个模块负责 `observed_windows` 表相关的读写逻辑，包括窗口上下文 hash
//! 计算、窗口记录读取、窗口列表查询，以及供 `input` 模块插入事件时使用的
//! upsert 能力。窗口身份由 `app_name`、`process_id`、`process_path` 和
//! `title` 四个字段共同决定，位置和尺寸只作为最近观察到的快照信息更新。

use crate::api::common::{
    append_where_clause, collect_rows, map_observed_window_record, resolve_pagination, stable_hash,
    PaginatedResponse, SortDirection,
};
use crate::errors::DatabaseResult;
use crate::models::{ObservedWindow, ObservedWindowRecord};
use chrono::{DateTime, Utc};
use duckdb::{params, params_from_iter, Connection, OptionalExt, ToSql};
use serde::{Deserialize, Serialize};

/// 窗口查询条件。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObservedWindowQuery {
    /// 页码，从 1 开始；未传时默认为第 1 页。
    pub page: Option<i64>,
    /// 每页条数；未传时使用默认值，过大时会被限制到 API 允许的最大值。
    pub size: Option<i64>,
    /// 零基 offset 游标；传入后优先于 page 决定查询起点。
    pub cursor: Option<i64>,
    pub app_name: Option<String>,
    pub context_hash: Option<String>,
    /// 排序字段；未传时按最近观察时间排序。
    pub sort_by: Option<ObservedWindowSortBy>,
    /// 排序方向；未传时降序。
    pub sort_direction: Option<SortDirection>,
}

/// 窗口查询支持的排序字段。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservedWindowSortBy {
    /// 按最近观察时间排序。
    #[default]
    LastSeenAt,
    /// 按首次观察时间排序。
    FirstSeenAt,
    /// 按窗口主键排序。
    WindowId,
    /// 按应用名称排序。
    AppName,
    /// 按关联事件数量排序。
    EventCount,
}

impl ObservedWindowSortBy {
    /// 返回排序字段对应的 SQL 表达式。
    fn as_sql(self) -> &'static str {
        match self {
            Self::LastSeenAt => "last_seen_at",
            Self::FirstSeenAt => "first_seen_at",
            Self::WindowId => "window_id",
            Self::AppName => "app_name",
            Self::EventCount => "event_count",
        }
    }
}

/// 通过窗口上下文四字段计算稳定 hash。
///
/// 注意：窗口位置和尺寸不参与 hash，因此同一个应用、进程、路径和标题下的多个
/// input 事件会归属到同一条 window 记录。
pub fn calculate_window_context_hash(window: &ObservedWindow) -> String {
    let process_id = window
        .process_id
        .map(|value| value.to_string())
        .unwrap_or_default();
    let process_path = window.process_path.as_deref().unwrap_or_default();

    stable_hash(&[
        window.app_name.as_str(),
        process_id.as_str(),
        process_path,
        window.title.as_str(),
    ])
}

/// 按主键读取窗口记录。
pub fn get_observed_window(
    conn: &Connection,
    window_id: i64,
) -> DatabaseResult<Option<ObservedWindowRecord>> {
    conn.query_row(
        r#"
        SELECT
            window_id,
            app_name,
            process_path,
            process_id,
            title,
            x,
            y,
            width,
            height,
            first_seen_at,
            last_seen_at,
            event_count,
            context_hash
        FROM observed_windows
        WHERE window_id = ?
        "#,
        params![window_id],
        map_observed_window_record,
    )
    .optional()
    .map_err(Into::into)
}

/// 按窗口上下文 hash 读取窗口记录。
pub fn get_observed_window_by_hash(
    conn: &Connection,
    context_hash: &str,
) -> DatabaseResult<Option<ObservedWindowRecord>> {
    conn.query_row(
        r#"
        SELECT
            window_id,
            app_name,
            process_path,
            process_id,
            title,
            x,
            y,
            width,
            height,
            first_seen_at,
            last_seen_at,
            event_count,
            context_hash
        FROM observed_windows
        WHERE context_hash = ?
        "#,
        params![context_hash],
        map_observed_window_record,
    )
    .optional()
    .map_err(Into::into)
}

/// 查询窗口记录。
pub fn query_observed_windows(
    conn: &Connection,
    query: &ObservedWindowQuery,
) -> DatabaseResult<PaginatedResponse<ObservedWindowRecord>> {
    let from_sql = " FROM observed_windows ";
    let mut sql = String::from(
        r#"
        SELECT
            window_id,
            app_name,
            process_path,
            process_id,
            title,
            x,
            y,
            width,
            height,
            first_seen_at,
            last_seen_at,
            event_count,
            context_hash
        "#,
    );
    sql.push_str(from_sql);

    let mut conditions = Vec::new();
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(app_name) = &query.app_name {
        conditions.push("app_name = ?");
        values.push(Box::new(app_name.clone()));
    }

    if let Some(context_hash) = &query.context_hash {
        conditions.push("context_hash = ?");
        values.push(Box::new(context_hash.clone()));
    }

    append_where_clause(&mut sql, &conditions);

    let mut count_sql = String::from("SELECT COUNT(*) ");
    count_sql.push_str(from_sql);
    append_where_clause(&mut count_sql, &conditions);

    let total: i64 = conn.query_row(
        &count_sql,
        params_from_iter(values.iter().map(|value| value.as_ref())),
        |row| row.get(0),
    )?;

    let (page, size, offset) = resolve_pagination(query.page, query.size, query.cursor);
    let sort_by = query.sort_by.unwrap_or_default().as_sql();
    let sort_direction = query.sort_direction.unwrap_or_default().as_sql();

    sql.push_str(&format!(
        " ORDER BY {sort_by} {sort_direction}, window_id {sort_direction} LIMIT ? OFFSET ?"
    ));
    values.push(Box::new(size));
    values.push(Box::new(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(
        params_from_iter(values.iter().map(|value| value.as_ref())),
        map_observed_window_record,
    )?;

    let list = collect_rows(rows)?;

    Ok(PaginatedResponse::new(page, size, total, list))
}

/// 按窗口上下文 hash 复用或创建窗口记录。
///
/// 这是 `input` 模块插入事件时使用的内部写接口：同一
/// `app_name/process_id/process_path/title` 组合会指向同一个 `window_id`。再次观察到
/// 已存在窗口时，只更新位置、尺寸和首末观察时间，不创建重复窗口记录。
pub(crate) fn upsert_observed_window(
    conn: &Connection,
    window: &ObservedWindow,
    observed_at: DateTime<Utc>,
) -> DatabaseResult<i64> {
    let context_hash = calculate_window_context_hash(window);

    if let Some(window_id) = conn
        .query_row(
            "SELECT window_id FROM observed_windows WHERE context_hash = ?",
            params![context_hash],
            |row| row.get(0),
        )
        .optional()?
    {
        conn.execute(
            r#"
            UPDATE observed_windows
            SET
                x = ?,
                y = ?,
                width = ?,
                height = ?,
                first_seen_at = LEAST(first_seen_at, ?),
                last_seen_at = GREATEST(last_seen_at, ?)
            WHERE window_id = ?
            "#,
            params![
                window.x,
                window.y,
                window.width,
                window.height,
                observed_at,
                observed_at,
                window_id,
            ],
        )?;

        return Ok(window_id);
    }

    let window_id = conn.query_row(
        r#"
        INSERT INTO observed_windows (
            app_name,
            process_path,
            process_id,
            title,
            x,
            y,
            width,
            height,
            first_seen_at,
            last_seen_at,
            context_hash
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING window_id
        "#,
        params![
            window.app_name,
            window.process_path,
            window.process_id,
            window.title,
            window.x,
            window.y,
            window.width,
            window.height,
            observed_at,
            observed_at,
            context_hash,
        ],
        |row| row.get(0),
    )?;

    Ok(window_id)
}
