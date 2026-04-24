use crate::server;
use database::db::{path::find_project_root, RunMode};
use database::{api::insert_input_event, db::DatabaseManager, models::InputEvent};
use log::{warn, LevelFilter};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, PhysicalSize, Runtime, Size, WindowEvent,
};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

const STDIO_EVENT_PREFIX: &str = "__HOROLOGION_INPUT_EVENT__";
const WINDOW_STATE_FILE: &str = "window-state.json";
const TRAY_ID: &str = "horologion-tray";
const TRAY_SHOW_ID: &str = "tray-show-window";
const TRAY_START_LISTENER_ID: &str = "tray-start-listener";
const TRAY_STOP_LISTENER_ID: &str = "tray-stop-listener";
const TRAY_QUIT_ID: &str = "tray-quit";
const APP_LOG_FILE: &str = "app.log";

static ALLOW_APP_EXIT: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct SavedWindowSize {
    width: u32,
    height: u32,
}

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

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .manage(server_state)
        .invoke_handler(server::router::handler())
        .setup(|app| {
            restore_main_window_size(app);
            watch_main_window_lifecycle(app);
            setup_tray(app)?;

            let state = app.state::<server::ServerState>();
            start_listener_sidecar(app.handle(), &state)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Reopen { .. } = event {
            show_main_window(app_handle);
        }
    });
}

pub fn start_listener_sidecar<R: Runtime>(
    app_handle: &AppHandle<R>,
    state: &server::ServerState,
) -> Result<(), String> {
    if state.is_listener_running() {
        return Ok(());
    }

    let sidecar_command = app_handle
        .shell()
        .sidecar("listener")
        .map_err(|error| error.to_string())?
        .env("HOROLOGION_LISTENER_TRANSPORT", "stdio");
    let (mut rx, child) = sidecar_command.spawn().map_err(|error| error.to_string())?;

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
}

pub fn stop_listener_sidecar(state: &server::ServerState) -> Result<(), String> {
    state.set_listener_running(false);

    if let Some(child) = state.take_listener_child() {
        child.kill().map_err(|error| error.to_string())?;
    }

    Ok(())
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
    let log_level = default_log_level();
    let log_file_path = match log_file_path(APP_LOG_FILE) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Failed to resolve app log path: {}", error);
            init_fallback_logger(log_level);
            return;
        }
    };

    if let Some(parent) = log_file_path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create app log directory: {}", error);
            init_fallback_logger(log_level);
            return;
        }
    }

    let log_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
    {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open app log file {:?}: {}", log_file_path, error);
            init_fallback_logger(log_level);
            return;
        }
    };

    let logger = fern::Dispatch::new()
        .level(log_level)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(std::io::stderr())
        .chain(log_file);

    if let Err(error) = logger.apply() {
        eprintln!("Failed to initialize app logger: {}", error);
        init_fallback_logger(log_level);
        return;
    }

    log::info!("App log file: {}", log_file_path.display());
}

fn default_log_level() -> LevelFilter {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        std::env::set_var("RUST_LOG", &log_level);
    }

    parse_log_level(
        &std::env::var("LOG_LEVEL")
            .or_else(|_| std::env::var("RUST_LOG"))
            .unwrap_or_else(|_| "info".to_string()),
    )
}

fn parse_log_level(value: &str) -> LevelFilter {
    let normalized = value
        .split(',')
        .next()
        .unwrap_or("info")
        .split('=')
        .next_back()
        .unwrap_or("info")
        .trim()
        .to_ascii_lowercase();

    match normalized.as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" | "warning" => LevelFilter::Warn,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    }
}

fn init_fallback_logger(level: LevelFilter) {
    let _ = fern::Dispatch::new()
        .level(level)
        .chain(std::io::stderr())
        .apply();
}

fn log_file_path(file_name: &str) -> Result<PathBuf, String> {
    let log_dir = match RunMode::from_env() {
        RunMode::Test => std::env::temp_dir().join("horologion").join("logs"),
        RunMode::Development => find_project_root()
            .map_err(|error| error.to_string())?
            .join("playground")
            .join("logs"),
        RunMode::Production => dirs::data_dir()
            .ok_or_else(|| "Failed to resolve system data directory".to_string())?
            .join("horologion")
            .join("logs"),
    };

    Ok(log_dir.join(file_name))
}

fn restore_main_window_size(app: &tauri::App) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let Some(size) = read_saved_window_size(window_state_path(app.handle())) else {
        return;
    };

    if let Err(error) = window.set_size(Size::Physical(PhysicalSize {
        width: size.width,
        height: size.height,
    })) {
        warn!("Failed to restore window size: {}", error);
    }
}

fn setup_tray(app: &tauri::App) -> Result<(), String> {
    let show = MenuItem::with_id(app, TRAY_SHOW_ID, "显示 Horologion", true, None::<&str>)
        .map_err(|error| error.to_string())?;
    let start_listener =
        MenuItem::with_id(app, TRAY_START_LISTENER_ID, "启动监听", true, None::<&str>)
            .map_err(|error| error.to_string())?;
    let stop_listener =
        MenuItem::with_id(app, TRAY_STOP_LISTENER_ID, "停止监听", true, None::<&str>)
            .map_err(|error| error.to_string())?;
    let separator = PredefinedMenuItem::separator(app).map_err(|error| error.to_string())?;
    let quit = MenuItem::with_id(app, TRAY_QUIT_ID, "退出 Horologion", true, None::<&str>)
        .map_err(|error| error.to_string())?;
    let menu = Menu::with_items(
        app,
        &[&show, &start_listener, &stop_listener, &separator, &quit],
    )
    .map_err(|error| error.to_string())?;

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("Horologion")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app_handle, event| {
            handle_tray_menu_event(app_handle, event.id().as_ref());
        })
        .on_tray_icon_event(|tray, event| {
            if should_show_window_from_tray_event(&event) {
                show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    }

    builder.build(app).map_err(|error| error.to_string())?;

    Ok(())
}

fn handle_tray_menu_event<R: Runtime>(app_handle: &AppHandle<R>, id: &str) {
    let state = app_handle.state::<server::ServerState>();

    match id {
        TRAY_SHOW_ID => show_main_window(app_handle),
        TRAY_START_LISTENER_ID => {
            if let Err(error) = start_listener_sidecar(app_handle, &state) {
                warn!("Failed to start listener from tray: {}", error);
            }
        }
        TRAY_STOP_LISTENER_ID => {
            if let Err(error) = stop_listener_sidecar(&state) {
                warn!("Failed to stop listener from tray: {}", error);
            }
        }
        TRAY_QUIT_ID => quit_from_tray(app_handle, &state),
        _ => {}
    }
}

fn should_show_window_from_tray_event(event: &TrayIconEvent) -> bool {
    matches!(
        event,
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } | TrayIconEvent::DoubleClick {
            button: MouseButton::Left,
            ..
        }
    )
}

fn show_main_window<R: Runtime>(app_handle: &AppHandle<R>) {
    let Some(window) = app_handle.get_webview_window("main") else {
        return;
    };

    if let Err(error) = window.show() {
        warn!("Failed to show main window: {}", error);
    }

    if let Err(error) = window.unminimize() {
        warn!("Failed to unminimize main window: {}", error);
    }

    if let Err(error) = window.set_focus() {
        warn!("Failed to focus main window: {}", error);
    }
}

fn quit_from_tray<R: Runtime>(app_handle: &AppHandle<R>, state: &server::ServerState) {
    ALLOW_APP_EXIT.store(true, Ordering::SeqCst);

    if let Err(error) = stop_listener_sidecar(state) {
        warn!("Failed to stop listener before quitting: {}", error);
    }

    app_handle.exit(0);
}

fn watch_main_window_lifecycle(app: &tauri::App) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let main_window = window.clone();
    let path = window_state_path(app.handle());

    window.on_window_event(move |event| match event {
        WindowEvent::Resized(size) => {
            if size.width < 400 || size.height < 300 {
                return;
            }

            let saved_size = SavedWindowSize {
                width: size.width,
                height: size.height,
            };

            if let Err(error) = write_saved_window_size(&path, saved_size) {
                warn!("Failed to persist window size: {}", error);
            }
        }
        WindowEvent::CloseRequested { api, .. } => {
            if ALLOW_APP_EXIT.load(Ordering::SeqCst) {
                return;
            }

            api.prevent_close();

            if let Err(error) = main_window.hide() {
                warn!("Failed to hide main window to tray: {}", error);
            }
        }
        _ => {}
    });
}

fn window_state_path<R: Runtime>(app_handle: &AppHandle<R>) -> Option<PathBuf> {
    app_handle
        .path()
        .app_config_dir()
        .map(|dir| dir.join(WINDOW_STATE_FILE))
        .map_err(|error| {
            warn!("Failed to resolve app config directory: {}", error);
            error
        })
        .ok()
}

fn read_saved_window_size(path: Option<PathBuf>) -> Option<SavedWindowSize> {
    let path = path?;
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn write_saved_window_size(path: &Option<PathBuf>, size: SavedWindowSize) -> Result<(), String> {
    let Some(path) = path else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let content = serde_json::to_string_pretty(&size).map_err(|error| error.to_string())?;
    std::fs::write(path, content).map_err(|error| error.to_string())
}
