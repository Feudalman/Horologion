//! 数据库操作封装模块

use crate::connection::{DatabaseManager, DatabaseResult};
use crate::schema::{AppUsageStats, InputEvent, WindowRecord};
use chrono::{DateTime, Local, TimeZone};
use duckdb::params;

/// 输入事件操作
pub struct InputEventOps;

impl InputEventOps {
    /// 插入输入事件
    pub fn insert(db: &DatabaseManager, event: &InputEvent) -> DatabaseResult<usize> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                INSERT INTO input_events (
                    timestamp, event_type, event_detail,
                    window_title, window_app_name, window_process_path, window_process_id,
                    window_position_x, window_position_y, window_size_width, window_size_height
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            )?;

            let affected = stmt.execute(params![
                event.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                event.event_type,
                event.event_detail,
                event.window_title,
                event.window_app_name,
                event.window_process_path,
                event.window_process_id.map(|id| id as i64),
                event.window_position_x,
                event.window_position_y,
                event.window_size_width,
                event.window_size_height,
            ])?;

            Ok(affected)
        })
    }

    /// 批量插入输入事件
    pub fn insert_batch(db: &DatabaseManager, events: &[InputEvent]) -> DatabaseResult<()> {
        db.with_connection(|conn| {
            let tx = conn.unchecked_transaction()?;

            let mut stmt = tx.prepare(
                r#"
                INSERT INTO input_events (
                    timestamp, event_type, event_detail,
                    window_title, window_app_name, window_process_path, window_process_id,
                    window_position_x, window_position_y, window_size_width, window_size_height
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            )?;

            for event in events {
                stmt.execute(params![
                    event.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    event.event_type,
                    event.event_detail,
                    event.window_title,
                    event.window_app_name,
                    event.window_process_path,
                    event.window_process_id.map(|id| id as i64),
                    event.window_position_x,
                    event.window_position_y,
                    event.window_size_width,
                    event.window_size_height,
                ])?;
            }

            tx.commit()?;
            Ok(())
        })
    }

    /// 根据时间范围查询事件
    pub fn find_by_time_range(
        db: &DatabaseManager,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> DatabaseResult<Vec<InputEvent>> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                SELECT id, timestamp, event_type, event_detail,
                       window_title, window_app_name, window_process_path, window_process_id,
                       window_position_x, window_position_y, window_size_width, window_size_height
                FROM input_events
                WHERE timestamp BETWEEN ?1 AND ?2
                ORDER BY timestamp
            "#,
            )?;

            let rows = stmt.query_map(
                params![
                    start.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    end.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
                ],
                |row| {
                    let timestamp_str: String = row.get(1)?;
                    let timestamp = chrono::NaiveDateTime::parse_from_str(
                        &timestamp_str,
                        "%Y-%m-%d %H:%M:%S%.3f",
                    )
                    .map_err(|e| duckdb::Error::InvalidColumnIndex(1))?;
                    let timestamp = Local
                        .from_local_datetime(&timestamp)
                        .single()
                        .ok_or_else(|| duckdb::Error::InvalidColumnIndex(1))?;
                    Ok(InputEvent {
                        id: Some(row.get(0)?),
                        timestamp,
                        event_type: row.get(2)?,
                        event_detail: row.get(3)?,
                        window_title: row.get(4)?,
                        window_app_name: row.get(5)?,
                        window_process_path: row.get(6)?,
                        window_process_id: row.get::<_, Option<i64>>(7)?.map(|id| id as u64),
                        window_position_x: row.get(8)?,
                        window_position_y: row.get(9)?,
                        window_size_width: row.get(10)?,
                        window_size_height: row.get(11)?,
                    })
                },
            )?;

            let events: Result<Vec<_>, _> = rows.collect();
            Ok(events?)
        })
    }

    /// 根据应用名称查询事件
    pub fn find_by_app(db: &DatabaseManager, app_name: &str) -> DatabaseResult<Vec<InputEvent>> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                SELECT id, timestamp, event_type, event_detail,
                       window_title, window_app_name, window_process_path, window_process_id,
                       window_position_x, window_position_y, window_size_width, window_size_height
                FROM input_events
                WHERE window_app_name = ?1
                ORDER BY timestamp DESC
                LIMIT 1000
            "#,
            )?;

            let rows = stmt.query_map([app_name], |row| {
                let timestamp_str: String = row.get(1)?;
                let timestamp =
                    chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S%.3f")
                        .map_err(|e| duckdb::Error::InvalidColumnIndex(1))?;
                let timestamp = Local
                    .from_local_datetime(&timestamp)
                    .single()
                    .ok_or_else(|| duckdb::Error::InvalidColumnIndex(1))?;
                Ok(InputEvent {
                    id: Some(row.get(0)?),
                    timestamp,
                    event_type: row.get(2)?,
                    event_detail: row.get(3)?,
                    window_title: row.get(4)?,
                    window_app_name: row.get(5)?,
                    window_process_path: row.get(6)?,
                    window_process_id: row.get::<_, Option<i64>>(7)?.map(|id| id as u64),
                    window_position_x: row.get(8)?,
                    window_position_y: row.get(9)?,
                    window_size_width: row.get(10)?,
                    window_size_height: row.get(11)?,
                })
            })?;

            let events: Result<Vec<_>, _> = rows.collect();
            Ok(events?)
        })
    }
}

/// 窗口记录操作
pub struct WindowRecordOps;

impl WindowRecordOps {
    /// 插入窗口记录
    pub fn insert(db: &DatabaseManager, record: &WindowRecord) -> DatabaseResult<usize> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                INSERT INTO window_records (
                    timestamp, title, app_name, process_path, process_id,
                    position_x, position_y, size_width, size_height, is_active
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            )?;

            let affected = stmt.execute(params![
                record.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                record.title,
                record.app_name,
                record.process_path,
                record.process_id as i64,
                record.position_x,
                record.position_y,
                record.size_width,
                record.size_height,
                record.is_active,
            ])?;

            Ok(affected)
        })
    }

    /// 获取最近的窗口记录
    pub fn get_recent(db: &DatabaseManager, limit: usize) -> DatabaseResult<Vec<WindowRecord>> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                SELECT id, timestamp, title, app_name, process_path, process_id,
                       position_x, position_y, size_width, size_height, is_active
                FROM window_records
                ORDER BY timestamp DESC
                LIMIT ?1
            "#,
            )?;

            let rows = stmt.query_map([limit], |row| {
                let timestamp_str: String = row.get(1)?;
                let timestamp =
                    chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S%.3f")
                        .map_err(|e| duckdb::Error::InvalidColumnIndex(1))?;
                let timestamp = Local
                    .from_local_datetime(&timestamp)
                    .single()
                    .ok_or_else(|| duckdb::Error::InvalidColumnIndex(1))?;
                Ok(WindowRecord {
                    id: Some(row.get(0)?),
                    timestamp,
                    title: row.get(2)?,
                    app_name: row.get(3)?,
                    process_path: row.get(4)?,
                    process_id: row.get::<_, i64>(5)? as u64,
                    position_x: row.get(6)?,
                    position_y: row.get(7)?,
                    size_width: row.get(8)?,
                    size_height: row.get(9)?,
                    is_active: row.get(10)?,
                })
            })?;

            let records: Result<Vec<_>, _> = rows.collect();
            Ok(records?)
        })
    }
}

/// 应用使用统计操作
pub struct AppUsageStatsOps;

impl AppUsageStatsOps {
    /// 更新或插入应用使用统计
    pub fn upsert(
        db: &DatabaseManager,
        app_name: &str,
        process_path: &str,
        session_time_seconds: i64,
    ) -> DatabaseResult<()> {
        db.with_connection(|conn| {
            let now = Local::now();

            // 尝试更新现有记录
            let mut stmt = conn.prepare(
                r#"
                UPDATE app_usage_stats 
                SET total_time_seconds = total_time_seconds + ?1,
                    session_count = session_count + 1,
                    last_used = ?2,
                    updated_at = ?3
                WHERE app_name = ?4 AND process_path = ?5
            "#,
            )?;

            let affected = stmt.execute(params![
                session_time_seconds,
                now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                app_name,
                process_path,
            ])?;

            // 如果没有更新任何记录，则插入新记录
            if affected == 0 {
                let mut insert_stmt = conn.prepare(
                    r#"
                    INSERT INTO app_usage_stats (
                        app_name, process_path, total_time_seconds, session_count,
                        last_used, created_at, updated_at
                    ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6)
                "#,
                )?;

                insert_stmt.execute(params![
                    app_name,
                    process_path,
                    session_time_seconds,
                    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                ])?;
            }

            Ok(())
        })
    }

    /// 获取使用时间最多的应用
    pub fn get_top_apps(db: &DatabaseManager, limit: usize) -> DatabaseResult<Vec<AppUsageStats>> {
        db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                r#"
                SELECT id, app_name, process_path, total_time_seconds, session_count,
                       last_used, created_at, updated_at
                FROM app_usage_stats
                ORDER BY total_time_seconds DESC
                LIMIT ?1
            "#,
            )?;

            let rows = stmt.query_map([limit], |row| {
                let last_used_str: String = row.get(5)?;
                let created_at_str: String = row.get(6)?;
                let updated_at_str: String = row.get(7)?;

                let last_used_naive =
                    chrono::NaiveDateTime::parse_from_str(&last_used_str, "%Y-%m-%d %H:%M:%S%.3f")
                        .map_err(|e| duckdb::Error::InvalidColumnIndex(5))?;
                let last_used = Local
                    .from_local_datetime(&last_used_naive)
                    .single()
                    .ok_or_else(|| duckdb::Error::InvalidColumnIndex(5))?;

                let created_at_naive =
                    chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                        .map_err(|e| duckdb::Error::InvalidColumnIndex(6))?;
                let created_at = Local
                    .from_local_datetime(&created_at_naive)
                    .single()
                    .ok_or_else(|| duckdb::Error::InvalidColumnIndex(6))?;

                let updated_at_naive =
                    chrono::NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S%.3f")
                        .map_err(|e| duckdb::Error::InvalidColumnIndex(7))?;
                let updated_at = Local
                    .from_local_datetime(&updated_at_naive)
                    .single()
                    .ok_or_else(|| duckdb::Error::InvalidColumnIndex(7))?;

                Ok(AppUsageStats {
                    id: Some(row.get(0)?),
                    app_name: row.get(1)?,
                    process_path: row.get(2)?,
                    total_time_seconds: row.get(3)?,
                    session_count: row.get(4)?,
                    last_used,
                    created_at,
                    updated_at,
                })
            })?;

            let stats: Result<Vec<_>, _> = rows.collect();
            Ok(stats?)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseConfig, RunMode};
    use crate::schema::SchemaManager;

    #[test]
    fn test_input_event_operations() {
        let config = DatabaseConfig::new(RunMode::Test).unwrap();
        let db = DatabaseManager::new(config);
        db.initialize().unwrap();
        SchemaManager::initialize_tables(&db).unwrap();

        let event = InputEvent {
            id: None,
            timestamp: Local::now(),
            event_type: "KeyPress".to_string(),
            event_detail: "A".to_string(),
            window_title: Some("Test Window".to_string()),
            window_app_name: Some("TestApp".to_string()),
            window_process_path: Some("/path/to/app".to_string()),
            window_process_id: Some(1234),
            window_position_x: Some(100.0),
            window_position_y: Some(200.0),
            window_size_width: Some(800.0),
            window_size_height: Some(600.0),
        };

        let affected = InputEventOps::insert(&db, &event).unwrap();
        assert!(affected > 0);

        let events = InputEventOps::find_by_app(&db, "TestApp").unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "KeyPress");
    }
}
