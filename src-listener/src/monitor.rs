use crate::permissions;
use crate::window::{get_current_window_info, WindowInfo};
use chrono::{DateTime, Local, Utc};
use database::{
    api::{init as init_database, insert_input_event},
    db::DatabaseManager,
    models::{InputEvent, InputEventKind, ObservedWindow},
};
use log::{error, info};
use rdev::{listen, Event, EventType};
use serde_json::json;
use std::io::{self, Write};
use std::panic::AssertUnwindSafe;

const COLLECTOR_NAME: &str = env!("CARGO_PKG_NAME");
const COLLECTOR_VERSION: &str = env!("CARGO_PKG_VERSION");
const TRANSPORT_ENV: &str = "HOROLOGION_LISTENER_TRANSPORT";
const STDIO_EVENT_PREFIX: &str = "__HOROLOGION_INPUT_EVENT__";

/// 事件监听器
pub struct EventListener {
    db: Option<DatabaseManager>,
    transport: ListenerTransport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListenerTransport {
    /// Standalone mode: write events directly into DuckDB.
    Database,
    /// Sidecar mode: stream JSON events to the parent Tauri process.
    Stdio,
}

impl ListenerTransport {
    fn from_env() -> Self {
        match std::env::var(TRANSPORT_ENV)
            .ok()
            .as_deref()
            .map(str::to_lowercase)
            .as_deref()
        {
            Some("stdio") => Self::Stdio,
            _ => Self::Database,
        }
    }
}

impl EventListener {
    /// 创建新的事件监听器
    pub fn new() -> Result<Self, String> {
        permissions::request_required_permissions();

        let transport = ListenerTransport::from_env();
        let db = if transport == ListenerTransport::Database {
            let db = DatabaseManager::from_env().map_err(|error| error.to_string())?;
            db.init().map_err(|error| error.to_string())?;
            db.with_connection(init_database)
                .map_err(|error| error.to_string())?;
            info!("Database schema initialized for listener");
            Some(db)
        } else {
            info!("Listener will stream events to stdout");
            None
        };

        Ok(Self { db, transport })
    }

    /// 启动监听
    pub fn start(&self) -> Result<(), String> {
        info!("Input listener process started");

        let db = self.db.clone();
        let transport = self.transport;
        if let Err(error) = listen(move |event| Self::callback(event, db.clone(), transport)) {
            let error_msg = format!("Listening error: {:?}", error);
            error!("{}", error_msg);
            return Err(error_msg);
        }

        Ok(())
    }

    /// 获取当前时间戳和窗口信息
    fn get_event_context(occurred_at: DateTime<Utc>) -> (String, String, Option<WindowInfo>) {
        let local_time = occurred_at.with_timezone(&Local);
        let time_str = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        // 获取当前活动窗口信息
        let window_info = get_current_window_info();
        let window_json = window_info
            .clone()
            .map(|w| w.to_json())
            .unwrap_or_else(|| "{}".to_string());

        (time_str, window_json, window_info)
    }

    /// 输出事件数据
    fn print_human_event(event_type: &str, event_detail: &str, time_str: &str, window_json: &str) {
        let event_data = format!(
            "{} --- {}: {} --- Window: {}",
            time_str, event_type, event_detail, window_json
        );
        println!("{}\n", event_data);
        io::stdout().flush().unwrap();
    }

    /// 监听回调
    fn callback(event: Event, db: Option<DatabaseManager>, transport: ListenerTransport) {
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            // 鼠标移动事件太频繁，直接跳过处理
            if matches!(event.event_type, EventType::MouseMove { .. }) {
                return;
            }

            let occurred_at = DateTime::<Utc>::from(event.time);
            let (time_str, window_json, window_info) = Self::get_event_context(occurred_at);

            match event.event_type {
                EventType::KeyPress(key) => {
                    let event_detail = format!("{:?}", key);
                    Self::print_event(
                        transport,
                        "KeyPress",
                        &event_detail,
                        &time_str,
                        &window_json,
                    );
                    Self::save_event(
                        db.as_ref(),
                        transport,
                        InputEventKind::KeyPress,
                        event_detail,
                        None,
                        None,
                        occurred_at,
                        window_info,
                        window_json,
                        event.name,
                    );
                }
                EventType::KeyRelease(key) => {
                    let event_detail = format!("{:?}", key);
                    Self::print_event(
                        transport,
                        "KeyRelease",
                        &event_detail,
                        &time_str,
                        &window_json,
                    );
                    Self::save_event(
                        db.as_ref(),
                        transport,
                        InputEventKind::KeyRelease,
                        event_detail,
                        None,
                        None,
                        occurred_at,
                        window_info,
                        window_json,
                        event.name,
                    );
                }
                EventType::ButtonPress(button) => {
                    let event_detail = format!("{:?}", button);
                    Self::print_event(
                        transport,
                        "ButtonPress",
                        &event_detail,
                        &time_str,
                        &window_json,
                    );
                    Self::save_event(
                        db.as_ref(),
                        transport,
                        InputEventKind::ButtonPress,
                        event_detail,
                        None,
                        None,
                        occurred_at,
                        window_info,
                        window_json,
                        event.name,
                    );
                }
                EventType::ButtonRelease(button) => {
                    let event_detail = format!("{:?}", button);
                    Self::print_event(
                        transport,
                        "ButtonRelease",
                        &event_detail,
                        &time_str,
                        &window_json,
                    );
                    Self::save_event(
                        db.as_ref(),
                        transport,
                        InputEventKind::ButtonRelease,
                        event_detail,
                        None,
                        None,
                        occurred_at,
                        window_info,
                        window_json,
                        event.name,
                    );
                }
                EventType::Wheel { delta_x, delta_y } => {
                    let event_detail = format!("delta_x:{}, delta_y:{}", delta_x, delta_y);
                    Self::print_event(transport, "Wheel", &event_detail, &time_str, &window_json);
                    Self::save_event(
                        db.as_ref(),
                        transport,
                        InputEventKind::Wheel,
                        event_detail,
                        Some(delta_x as f64),
                        Some(delta_y as f64),
                        occurred_at,
                        window_info,
                        window_json,
                        event.name,
                    );
                }
                _ => {}
            }
        }));

        if result.is_err() {
            error!("Error in event callback");
        }
    }

    fn print_event(
        transport: ListenerTransport,
        event_type: &str,
        event_detail: &str,
        time_str: &str,
        window_json: &str,
    ) {
        if transport == ListenerTransport::Database {
            Self::print_human_event(event_type, event_detail, time_str, window_json);
        }
    }

    /// 将监听事件写入数据库。
    fn save_event(
        db: Option<&DatabaseManager>,
        transport: ListenerTransport,
        kind: InputEventKind,
        value: String,
        delta_x: Option<f64>,
        delta_y: Option<f64>,
        occurred_at: DateTime<Utc>,
        window_info: Option<WindowInfo>,
        raw_window: String,
        key_name: Option<String>,
    ) {
        let raw_event = json!({
            "kind": kind.as_str(),
            "value": value,
            "delta_x": delta_x,
            "delta_y": delta_y,
            "key_name": key_name,
            "occurred_at": occurred_at,
            "collector_name": COLLECTOR_NAME,
            "collector_version": COLLECTOR_VERSION,
        })
        .to_string();

        let input_event = InputEvent {
            occurred_at,
            kind,
            value,
            delta_x,
            delta_y,
            window: window_info.map(Self::map_window_info),
            raw_event: Some(raw_event),
            raw_window: Some(raw_window),
            collector_name: COLLECTOR_NAME.to_string(),
            collector_version: COLLECTOR_VERSION.to_string(),
        };

        match transport {
            ListenerTransport::Database => {
                let Some(db) = db else {
                    error!("Database transport selected without an initialized database");
                    return;
                };

                if let Err(error) =
                    db.with_connection(|conn| insert_input_event(conn, &input_event))
                {
                    error!("Failed to save input event: {}", error);
                }
            }
            ListenerTransport::Stdio => match serde_json::to_string(&input_event) {
                Ok(payload) => {
                    println!("{STDIO_EVENT_PREFIX}{payload}");
                    io::stdout().flush().unwrap();
                }
                Err(error) => error!("Failed to serialize input event: {}", error),
            },
        }
    }

    /// 将 listener 的窗口结构转换为数据库模型。
    fn map_window_info(window: WindowInfo) -> ObservedWindow {
        ObservedWindow {
            app_name: window.app_name,
            process_path: non_empty_string(window.process_path),
            process_id: Some(window.process_id),
            title: window.title,
            x: Some(window.position.0),
            y: Some(window.position.1),
            width: Some(window.size.0),
            height: Some(window.size.1),
        }
    }
}

fn non_empty_string(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
