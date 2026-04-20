//! 数据库公共 API。
//!
//! 这里提供给 `src-listener` 和 `src-tauri` 共同使用的读写接口：
//! - `src-listener` 通过插入接口写入输入事件和活动窗口快照；
//! - `src-tauri` 通过读取接口查询窗口、事件，以及二者的关联结果。

mod common;

pub mod input;
pub mod window;

pub use common::{PaginatedResponse, SortDirection};
pub use input::{get_input_event, insert_input_event, insert_input_events, query_input_events};
pub use input::{InputEventQuery, InputEventSortBy, InputEventWithWindow};
pub use window::{
    calculate_window_context_hash, get_observed_window, get_observed_window_by_hash,
    query_observed_windows, ObservedWindowQuery, ObservedWindowSortBy,
};

use crate::errors::DatabaseResult;
use crate::models::init_schema;
use duckdb::Connection;

/// 初始化数据库 schema。
///
/// 调用方可以直接使用 `models::init_schema`，这里保留一个 API 层入口，方便
/// listener/tauri 只依赖 api 模块完成初始化。
pub fn init(conn: &Connection) -> DatabaseResult<()> {
    init_schema(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{connect, DatabaseConfig, RunMode};
    use crate::models::{InputEvent, InputEventKind, ObservedWindow};
    use chrono::{DateTime, Utc};

    fn setup() -> Connection {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let conn = connect(&config).unwrap();
        init(&conn).unwrap();
        conn
    }

    fn window(title: &str) -> ObservedWindow {
        ObservedWindow {
            app_name: "Terminal".to_string(),
            process_path: Some("/Applications/Utilities/Terminal.app".to_string()),
            process_id: Some(42),
            title: title.to_string(),
            x: Some(0.0),
            y: Some(0.0),
            width: Some(1280.0),
            height: Some(720.0),
        }
    }

    fn event(value: &str, window: Option<ObservedWindow>) -> InputEvent {
        InputEvent {
            occurred_at: DateTime::parse_from_rfc3339("2026-04-18T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            kind: InputEventKind::KeyPress,
            value: value.to_string(),
            delta_x: None,
            delta_y: None,
            window,
            raw_event: Some(format!(r#"{{"value":"{}"}}"#, value)),
            raw_window: None,
        }
    }

    #[test]
    fn window_hash_uses_identity_fields_only() {
        let mut first = window("cargo test");
        let mut second = first.clone();
        second.x = Some(100.0);
        second.width = Some(800.0);

        assert_eq!(
            calculate_window_context_hash(&first),
            calculate_window_context_hash(&second)
        );

        first.title = "cargo run".to_string();
        assert_ne!(
            calculate_window_context_hash(&first),
            calculate_window_context_hash(&second)
        );
    }

    #[test]
    fn insert_input_events_reuses_window_by_context_hash() {
        let conn = setup();
        let mut later_window = window("cargo test");
        later_window.x = Some(24.0);

        let first = insert_input_event(&conn, &event("KeyA", Some(window("cargo test")))).unwrap();
        let second = insert_input_event(&conn, &event("KeyB", Some(later_window))).unwrap();

        assert_eq!(first.window_id, second.window_id);

        let window = get_observed_window(&conn, first.window_id.unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(window.event_count, 2);
        assert_eq!(window.x, Some(24.0));

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM observed_windows", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn query_input_events_returns_joined_window() {
        let conn = setup();
        insert_input_event(&conn, &event("KeyA", Some(window("cargo test")))).unwrap();
        insert_input_event(&conn, &event("KeyB", None)).unwrap();

        let records = query_input_events(
            &conn,
            &InputEventQuery {
                kind: Some(InputEventKind::KeyPress),
                size: Some(10),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(records.page, 1);
        assert_eq!(records.total, 2);
        assert_eq!(records.pages, 1);
        assert_eq!(records.list.len(), 2);
        assert!(records.list.iter().any(|record| record.window.is_some()));
        assert!(records.list.iter().any(|record| record.window.is_none()));
    }

    #[test]
    fn query_input_events_supports_page_size_and_cursor() {
        let conn = setup();
        insert_input_event(&conn, &event("KeyA", Some(window("cargo test")))).unwrap();
        insert_input_event(&conn, &event("KeyB", Some(window("cargo test")))).unwrap();
        insert_input_event(&conn, &event("KeyC", Some(window("cargo test")))).unwrap();

        let second_page = query_input_events(
            &conn,
            &InputEventQuery {
                page: Some(2),
                size: Some(1),
                sort_by: Some(InputEventSortBy::EventId),
                sort_direction: Some(SortDirection::Asc),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(second_page.page, 2);
        assert_eq!(second_page.total, 3);
        assert_eq!(second_page.pages, 3);
        assert_eq!(second_page.list.len(), 1);
        assert_eq!(second_page.list[0].event.value, "KeyB");

        let cursor_page = query_input_events(
            &conn,
            &InputEventQuery {
                page: Some(1),
                size: Some(1),
                cursor: Some(2),
                sort_by: Some(InputEventSortBy::EventId),
                sort_direction: Some(SortDirection::Asc),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(cursor_page.page, 3);
        assert_eq!(cursor_page.total, 3);
        assert_eq!(cursor_page.pages, 3);
        assert_eq!(cursor_page.list.len(), 1);
        assert_eq!(cursor_page.list[0].event.value, "KeyC");
    }

    #[test]
    fn query_observed_windows_can_filter_by_hash() {
        let conn = setup();
        let observed_window = window("cargo test");
        let context_hash = calculate_window_context_hash(&observed_window);
        insert_input_event(&conn, &event("KeyA", Some(observed_window))).unwrap();

        let records = query_observed_windows(
            &conn,
            &ObservedWindowQuery {
                context_hash: Some(context_hash.clone()),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(records.page, 1);
        assert_eq!(records.total, 1);
        assert_eq!(records.pages, 1);
        assert_eq!(records.list.len(), 1);
        assert_eq!(records.list[0].context_hash, context_hash);
    }

    #[test]
    fn query_observed_windows_supports_page_size() {
        let conn = setup();
        insert_input_event(&conn, &event("KeyA", Some(window("cargo test")))).unwrap();
        insert_input_event(&conn, &event("KeyB", Some(window("cargo run")))).unwrap();

        let records = query_observed_windows(
            &conn,
            &ObservedWindowQuery {
                page: Some(2),
                size: Some(1),
                sort_by: Some(ObservedWindowSortBy::WindowId),
                sort_direction: Some(SortDirection::Asc),
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(records.page, 2);
        assert_eq!(records.total, 2);
        assert_eq!(records.pages, 2);
        assert_eq!(records.list.len(), 1);
        assert_eq!(records.list[0].title, "cargo run");
    }
}
