//! 持久化模型的表结构初始化。

use crate::errors::DatabaseResult;
use duckdb::Connection;

pub const TABLE_OBSERVED_WINDOWS: &str = "observed_windows";
pub const TABLE_INPUT_EVENTS: &str = "input_events";

/// 当前模块管理的数据表版本。
pub const SCHEMA_VERSION: i32 = 1;

/// 创建 `models` 模块所有数据表的 SQL。
pub const SCHEMA_SQL: &str = include_str!("schema.sql");

/// 创建或升级监听数据持久化所需的数据表。
pub fn init_schema(conn: &Connection) -> DatabaseResult<()> {
    conn.execute_batch(SCHEMA_SQL)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{connect, DatabaseConfig, RunMode};

    #[test]
    fn init_schema_creates_listener_tables() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let conn = connect(&config).unwrap();

        init_schema(&conn).unwrap();

        let table_count: i64 = conn
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM information_schema.tables
                WHERE table_name IN ('observed_windows', 'input_events')
                "#,
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_count, 2);
    }

    #[test]
    fn init_schema_allows_basic_event_insert() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let conn = connect(&config).unwrap();

        init_schema(&conn).unwrap();

        conn.execute_batch(
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
            VALUES (
                'Terminal',
                '/Applications/Utilities/Terminal.app',
                42,
                'cargo test',
                0,
                0,
                1280,
                720,
                TIMESTAMPTZ '2026-04-18 00:00:00+00',
                TIMESTAMPTZ '2026-04-18 00:00:00+00',
                'terminal-42-cargo-test'
            );

            INSERT INTO input_events (
                occurred_at,
                event_kind,
                event_value,
                window_id,
                raw_event,
                raw_window
            )
            VALUES (
                TIMESTAMPTZ '2026-04-18 00:00:00+00',
                'key_press',
                'KeyA',
                1,
                '{"kind":"KeyPress","value":"KeyA"}',
                '{"app_name":"Terminal"}'
            );
            "#,
        )
        .unwrap();

        let event_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM input_events", [], |row| row.get(0))
            .unwrap();

        assert_eq!(event_count, 1);
    }
}
