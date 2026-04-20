//! `api` 模块内部共用的数据库辅助函数。
//!
//! 这个模块不作为外部公共接口暴露，主要服务于 `input` 和 `window` 子模块：
//! - 统一 DuckDB 查询结果到业务记录模型的映射逻辑；
//! - 提供分页参数归一化和动态 `WHERE` 子句拼接；
//! - 保存输入事件类型解析和窗口上下文 hash 的底层实现。

use crate::errors::{DatabaseError, DatabaseResult};
use crate::models::{InputEventKind, InputEventRecord, ObservedWindowRecord};
use duckdb::{types::Type, Row};
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PAGE_SIZE: i64 = 100;
const MAX_PAGE_SIZE: i64 = 1_000;

/// 统一分页查询返回格式。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// 当前页码，从 1 开始。使用 cursor 查询时会根据 cursor 和 size 反推当前页。
    pub page: i64,
    /// 当前过滤条件下的总记录数。
    pub total: i64,
    /// 当前过滤条件和分页大小下的总页数。
    pub pages: i64,
    /// 当前页查询到的数据列表。
    pub list: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    /// 根据分页信息和查询结果创建统一分页返回值。
    pub(crate) fn new(page: i64, size: i64, total: i64, list: Vec<T>) -> Self {
        let pages = if total == 0 {
            0
        } else {
            (total + size - 1) / size
        };

        Self {
            page,
            total,
            pages,
            list,
        }
    }
}

/// 查询排序方向。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// 升序。
    Asc,
    /// 降序。
    #[default]
    Desc,
}

impl SortDirection {
    /// 返回排序方向对应的 SQL 关键字。
    pub(crate) fn as_sql(self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

/// 在动态 SQL 后追加 `WHERE` 子句。
///
/// 查询模块会先按入参收集过滤条件，再由这里统一拼接，避免各查询函数重复处理
/// 空条件和 `AND` 连接逻辑。
pub(crate) fn append_where_clause(sql: &mut String, conditions: &[&str]) {
    if conditions.is_empty() {
        return;
    }

    sql.push_str(" WHERE ");
    sql.push_str(&conditions.join(" AND "));
}

/// 归一化分页参数。
///
/// page 从 1 开始；size 会被限制在 API 支持的范围内；cursor 当前表示零基 offset 游标。
/// 传入 cursor 时，cursor 优先于 page 决定查询 offset。
pub(crate) fn resolve_pagination(
    page: Option<i64>,
    size: Option<i64>,
    cursor: Option<i64>,
) -> (i64, i64, i64) {
    let size = size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, MAX_PAGE_SIZE);

    if let Some(cursor) = cursor {
        let offset = cursor.max(0);
        let page = (offset / size) + 1;
        return (page, size, offset);
    }

    let page = page.unwrap_or(DEFAULT_PAGE).max(1);
    let offset = (page - 1) * size;

    (page, size, offset)
}

/// 收集 DuckDB 的映射结果为普通 `Vec`。
///
/// DuckDB 的 `query_map` 返回惰性迭代器；API 层对外返回完整记录列表，
/// 并在这里将任意行级错误统一转换成 `DatabaseResult`。
pub(crate) fn collect_rows<T>(
    rows: duckdb::MappedRows<'_, impl FnMut(&Row<'_>) -> duckdb::Result<T>>,
) -> DatabaseResult<Vec<T>> {
    let mut records = Vec::new();

    for row in rows {
        records.push(row?);
    }

    Ok(records)
}

/// 将 `input_events` 查询结果映射为 `InputEventRecord`。
///
/// 调用方需要确保 SQL 选择的列顺序与该函数中的索引一致。
pub(crate) fn map_input_event_record(row: &Row<'_>) -> duckdb::Result<InputEventRecord> {
    let kind: String = row.get(2)?;

    Ok(InputEventRecord {
        event_id: row.get(0)?,
        occurred_at: row.get(1)?,
        kind: parse_input_event_kind(&kind)?,
        value: row.get(3)?,
        delta_x: row.get(4)?,
        delta_y: row.get(5)?,
        window_id: row.get(6)?,
        raw_event: row.get(7)?,
        raw_window: row.get(8)?,
        created_at: row.get(9)?,
    })
}

/// 将 `observed_windows` 查询结果映射为 `ObservedWindowRecord`。
///
/// 调用方需要确保 SQL 选择的列顺序与该函数中的索引一致。
pub(crate) fn map_observed_window_record(row: &Row<'_>) -> duckdb::Result<ObservedWindowRecord> {
    Ok(ObservedWindowRecord {
        window_id: row.get(0)?,
        app_name: row.get(1)?,
        process_path: row.get(2)?,
        process_id: row.get(3)?,
        title: row.get(4)?,
        x: row.get(5)?,
        y: row.get(6)?,
        width: row.get(7)?,
        height: row.get(8)?,
        first_seen_at: row.get(9)?,
        last_seen_at: row.get(10)?,
        event_count: row.get(11)?,
        context_hash: row.get(12)?,
    })
}

/// 将数据库中保存的事件类型字符串还原为业务枚举。
///
/// schema 通过 CHECK 约束限制了合法值；如果遇到未知字符串，说明数据和模型已经
/// 不一致，应作为读取转换错误暴露出来。
pub(crate) fn parse_input_event_kind(kind: &str) -> duckdb::Result<InputEventKind> {
    match kind {
        "key_press" => Ok(InputEventKind::KeyPress),
        "key_release" => Ok(InputEventKind::KeyRelease),
        "button_press" => Ok(InputEventKind::ButtonPress),
        "button_release" => Ok(InputEventKind::ButtonRelease),
        "wheel" => Ok(InputEventKind::Wheel),
        value => Err(duckdb::Error::FromSqlConversionFailure(
            2,
            Type::Text,
            Box::new(DatabaseError::InvalidConfig(format!(
                "unknown input event kind: {}",
                value
            ))),
        )),
    }
}

/// 计算稳定的 FNV-1a 风格 hash 字符串。
///
/// 每段内容都会先写入长度再写入字节，避免不同字段组合拼接后产生歧义
/// 返回值带有版本前缀，后续如果 hash 规则升级可以并存区分。
pub(crate) fn stable_hash(parts: &[&str]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;

    for part in parts {
        for byte in part.len().to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }

    format!("v1-{hash:016x}")
}
