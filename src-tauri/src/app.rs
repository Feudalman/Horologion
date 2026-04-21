use crate::server;
use database::{api::insert_input_event, db::DatabaseManager, models::InputEvent};
use log::warn;
use std::sync::atomic::Ordering;
use tauri::Manager;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

const STDIO_EVENT_PREFIX: &str = "__HOROLOGION_INPUT_EVENT__";

/// 初始化并运行应用
pub fn init_and_run() {
    // 加载 .env 配置文件
    dotenvy::dotenv().ok();

    // 初始化日志设置
    init_log();

    // server state 需要早于 Tauri builder 初始化，确保 command 注册后即可访问数据库。
    let server_state = server::ServerState::from_env().unwrap_or_else(|error| {
        eprintln!("Failed to initialize server state: {}", error);
        std::process::exit(1);
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(server_state)
        .invoke_handler(server::router::handler())
        .setup(|app| {
            let sidecar_command = app
                .shell()
                .sidecar("listener")
                .unwrap()
                .env("HOROLOGION_LISTENER_TRANSPORT", "stdio");
            let (mut rx, child) = sidecar_command.spawn().expect("Failed to spawn sidecar");
            let state = app.state::<server::ServerState>();

            // 保存 child 句柄并记录运行状态，否则 setup 返回后 sidecar 可能失去生命周期管理。
            state.set_listener_running(true);
            state.set_listener_child(child);
            let listener_running = state.listener_running_handle();
            let db = state.db().clone();

            tauri::async_runtime::spawn(async move {
                let mut stdout_buffer = String::new();
                // 读取诸如 stdout 之类的事件
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            handle_listener_stdout(&db, &line, &mut stdout_buffer);
                        }
                        CommandEvent::Stderr(line) => {
                            // 打印错误流到主程序终端
                            warn!("[Sidecar STDERR]: {}", String::from_utf8_lossy(&line));
                        }
                        CommandEvent::Terminated(payload) => {
                            listener_running.store(false, Ordering::SeqCst);
                            warn!("[Sidecar] Terminated: {:?}", payload.code);
                        }
                        _ => {}
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_listener_stdout(db: &DatabaseManager, chunk: &[u8], buffer: &mut String) {
    buffer.push_str(&String::from_utf8_lossy(chunk));

    while let Some(line_end) = buffer.find('\n') {
        let line = buffer.drain(..=line_end).collect::<String>();
        save_listener_event(db, line.trim());
    }
}

fn save_listener_event(db: &DatabaseManager, payload: &str) {
    if payload.is_empty() {
        return;
    }

    let Some(payload) = payload.strip_prefix(STDIO_EVENT_PREFIX) else {
        return;
    };

    match serde_json::from_str::<InputEvent>(payload) {
        Ok(input_event) => {
            if let Err(error) = db.with_connection(|conn| insert_input_event(conn, &input_event)) {
                warn!("Failed to save sidecar input event: {}", error);
            }
        }
        Err(error) => {
            warn!(
                "Failed to parse sidecar input event: {}; payload: {}",
                error, payload
            );
        }
    }
}

pub fn init_log() {
    // 检查是否手动设置了日志级别，如果没有，则从配置文件读取，默认为 info
    if std::env::var("RUST_LOG").is_err() {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        std::env::set_var("RUST_LOG", log_level);
    }
    // 初始化日志
    env_logger::init();
}
