//! 输入事件读写 API。
//!
//! 这个模块面向 `src-listener` 提供 input 事件插入接口，并面向 `src-tauri` 提供 input 事件读取和筛选接口。
//! 插入时如果事件携带窗口快照，会调用 `window` 模块按窗口上下文 hash 复用或创建窗口记录，
//! 从而支持一条 window 记录关联多条 input 记录。

use crate::api::common::{
    append_where_clause, collect_rows, map_input_event_record, resolve_pagination,
    PaginatedResponse, SortDirection,
};
use crate::api::window::upsert_observed_window;
use crate::errors::{DatabaseError, DatabaseResult};
use crate::models::{InputEvent, InputEventKind, InputEventRecord, ObservedWindowRecord};

use chrono::{DateTime, Utc};
use duckdb::{params, params_from_iter, Connection, OptionalExt, Row, ToSql};
use serde::{Deserialize, Serialize};

/// 输入事件及其关联窗口记录。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputEventWithWindow {
    pub event: InputEventRecord,
    pub window: Option<ObservedWindowRecord>,
}

/// 输入事件查询条件。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InputEventQuery {
    /// 页码，从 1 开始；未传时默认为第 1 页。
    pub page: Option<i64>,
    /// 每页条数；未传时使用默认值，过大时会被限制到 API 允许的最大值。
    pub size: Option<i64>,
    /// 零基 offset 游标；传入后优先于 page 决定查询起点。
    pub cursor: Option<i64>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub kind: Option<InputEventKind>,
    pub window_id: Option<i64>,
    pub app_name: Option<String>,
    pub context_hash: Option<String>,
    /// 排序字段；未传时按事件发生时间排序。
    pub sort_by: Option<InputEventSortBy>,
    /// 排序方向；未传时降序。
    pub sort_direction: Option<SortDirection>,
}

/// 输入事件查询支持的排序字段。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputEventSortBy {
    /// 按事件发生时间排序。
    #[default]
    OccurredAt,
    /// 按事件主键排序。
    EventId,
    /// 按事件类型排序。
    Kind,
    /// 按事件值排序。
    Value,
    /// 按关联窗口应用名排序。
    AppName,
}

impl InputEventSortBy {
    /// 返回排序字段对应的 SQL 表达式。
    fn as_sql(self) -> &'static str {
        match self {
            Self::OccurredAt => "e.occurred_at",
            Self::EventId => "e.event_id",
            Self::Kind => "e.event_kind",
            Self::Value => "e.event_value",
            Self::AppName => "w.app_name",
        }
    }
}

/// 插入一条输入事件。
///
/// 如果事件携带窗口信息，会先按 `app_name/process_id/process_path/title` 计算 `context_hash`，然后复用或创建 `observed_windows` 记录。
/// 多个 input 事件可以指向同一条 window 记录。
pub fn insert_input_event(
    conn: &Connection,
    event: &InputEvent,
) -> DatabaseResult<InputEventRecord> {
    let window_id = event
        .window
        .as_ref()
        .map(|window| upsert_observed_window(conn, window, event.occurred_at))
        .transpose()?;

    let event_id: i64 = conn.query_row(
        r#"
        INSERT INTO input_events (
            occurred_at,
            event_kind,
            event_value,
            delta_x,
            delta_y,
            window_id,
            raw_event,
            raw_window
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING event_id
        "#,
        params![
            event.occurred_at,
            event.kind.as_str(),
            event.value,
            event.delta_x,
            event.delta_y,
            window_id,
            event.raw_event,
            event.raw_window,
        ],
        |row| row.get(0),
    )?;

    // 如果关联了窗口，事件插入后窗口的 event_count 也需要更新
    if let Some(window_id) = window_id {
        conn.execute(
            "UPDATE observed_windows SET event_count = event_count + 1 WHERE window_id = ?",
            params![window_id],
        )?;
    }

    get_input_event(conn, event_id)?.ok_or_else(|| {
        DatabaseError::InvalidConfig(format!(
            "inserted input event {} could not be loaded",
            event_id
        ))
    })
}

/// 批量插入输入事件。
pub fn insert_input_events(
    conn: &Connection,
    events: &[InputEvent],
) -> DatabaseResult<Vec<InputEventRecord>> {
    events
        .iter()
        .map(|event| insert_input_event(conn, event))
        .collect()
}

/// 按主键读取输入事件。
pub fn get_input_event(
    conn: &Connection,
    event_id: i64,
) -> DatabaseResult<Option<InputEventRecord>> {
    conn.query_row(
        r#"
        SELECT
            event_id,
            occurred_at,
            event_kind,
            event_value,
            delta_x,
            delta_y,
            window_id,
            raw_event,
            raw_window,
            created_at
        FROM input_events
        WHERE event_id = ?
        "#,
        params![event_id],
        map_input_event_record,
    )
    .optional()
    .map_err(Into::into)
}

/// 查询输入事件，并附带其关联窗口记录。
pub fn query_input_events(
    conn: &Connection,
    query: &InputEventQuery,
) -> DatabaseResult<PaginatedResponse<InputEventWithWindow>> {
    let from_sql = r#"
        FROM input_events e
        LEFT JOIN observed_windows w ON e.window_id = w.window_id
        "#;
    let mut sql = String::from(
        r#"
        SELECT
            e.event_id,
            e.occurred_at,
            e.event_kind,
            e.event_value,
            e.delta_x,
            e.delta_y,
            e.window_id,
            e.raw_event,
            e.raw_window,
            e.created_at,
            w.window_id,
            w.app_name,
            w.process_path,
            w.process_id,
            w.title,
            w.x,
            w.y,
            w.width,
            w.height,
            w.first_seen_at,
            w.last_seen_at,
            w.event_count,
            w.context_hash
        "#,
    );
    sql.push_str(from_sql);

    let mut conditions = Vec::new();
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();

    // 处理查询条件，支持同时按事件属性和窗口属性筛选

    if let Some(start_at) = query.start_at {
        conditions.push("e.occurred_at >= ?");
        values.push(Box::new(start_at));
    }

    if let Some(end_at) = query.end_at {
        conditions.push("e.occurred_at <= ?");
        values.push(Box::new(end_at));
    }

    if let Some(kind) = query.kind {
        conditions.push("e.event_kind = ?");
        values.push(Box::new(kind.as_str().to_string()));
    }

    if let Some(window_id) = query.window_id {
        conditions.push("e.window_id = ?");
        values.push(Box::new(window_id));
    }

    if let Some(app_name) = &query.app_name {
        conditions.push("w.app_name = ?");
        values.push(Box::new(app_name.clone()));
    }

    if let Some(context_hash) = &query.context_hash {
        conditions.push("w.context_hash = ?");
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
        " ORDER BY {sort_by} {sort_direction}, e.event_id {sort_direction} LIMIT ? OFFSET ?"
    ));
    values.push(Box::new(size));
    values.push(Box::new(offset));

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(
        params_from_iter(values.iter().map(|value| value.as_ref())),
        map_input_event_with_window,
    )?;

    let list = collect_rows(rows)?;

    Ok(PaginatedResponse::new(page, size, total, list))
}

/// 将 input 与 window 的 LEFT JOIN 查询结果映射为组合记录。
///
/// 前 10 列复用 `map_input_event_record` 的 input 映射规则，
/// 后续列在存在 `window_id` 时组装成关联窗口记录；无窗口事件会保留 `window: None`。
fn map_input_event_with_window(row: &Row<'_>) -> duckdb::Result<InputEventWithWindow> {
    let event = map_input_event_record(row)?;
    let window_id: Option<i64> = row.get(10)?;
    let window = if window_id.is_some() {
        Some(ObservedWindowRecord {
            window_id: row.get(10)?,
            app_name: row.get(11)?,
            process_path: row.get(12)?,
            process_id: row.get(13)?,
            title: row.get(14)?,
            x: row.get(15)?,
            y: row.get(16)?,
            width: row.get(17)?,
            height: row.get(18)?,
            first_seen_at: row.get(19)?,
            last_seen_at: row.get(20)?,
            event_count: row.get(21)?,
            context_hash: row.get(22)?,
        })
    } else {
        None
    };

    Ok(InputEventWithWindow { event, window })
}
