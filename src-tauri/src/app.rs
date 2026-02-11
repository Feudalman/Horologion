use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

/// 监听器管理器
struct ListenerManager {
    sidecar_child: Option<tauri_plugin_shell::process::CommandChild>,
}

/// 实现监听器管理器
impl ListenerManager {
    fn new() -> Self {
        Self {
            sidecar_child: None,
        }
    }

    /// 启动监听器 sidecar 进程
    fn start(&mut self, app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // 检查是否已经启动，不允许重复启动
        if self.sidecar_child.is_some() {
            warn!("Listener sidecar is already running");
            return Ok(());
        }

        info!("Starting listener sidecar process...");

        // 使用 Tauri 的 sidecar API 启动子进程
        let sidecar_command = app_handle.shell().sidecar("listener")?;
        let (mut rx, child) = sidecar_command.spawn()?;

        info!("Listener sidecar started with PID: {:?}", child.pid());

        // 在后台线程中处理 sidecar 输出
        let app_handle_clone = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    tauri_plugin_shell::process::CommandEvent::Stdout(data) => {
                        let output = String::from_utf8_lossy(&data);
                        for line in output.lines() {
                            if !line.trim().is_empty() {
                                info!("Listener output: {}", line);
                                handle_listener_event(line, &app_handle_clone);
                            }
                        }
                    }
                    tauri_plugin_shell::process::CommandEvent::Stderr(data) => {
                        let error_output = String::from_utf8_lossy(&data);
                        for line in error_output.lines() {
                            if !line.trim().is_empty() {
                                warn!("Listener stderr: {}", line);
                            }
                        }
                    }
                    tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                        info!("Listener sidecar terminated with code: {:?}", payload.code);
                        break;
                    }
                    _ => {}
                }
            }
        });

        self.sidecar_child = Some(child);
        Ok(())
    }

    /// 停止监听器 sidecar 进程
    fn stop(&mut self) {
        if let Some(child) = self.sidecar_child.take() {
            match child.kill() {
                Ok(_) => info!("Listener sidecar terminated successfully"),
                Err(e) => error!("Failed to terminate listener sidecar: {}", e),
            }
        }
    }
}

/// 全局的监听器管理器
static LISTENER: Lazy<Mutex<ListenerManager>> = Lazy::new(|| Mutex::new(ListenerManager::new()));

/// 初始化并运行应用
pub fn init_and_run() {
    // 检查是否手动设置了日志级别，如果没有，则设置为 info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 启动监听器 sidecar 进程
            if let Err(e) = start_listener_sidecar(app.handle()) {
                error!("Failed to start listener sidecar: {}", e);
            }
            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                cleanup_listener_process();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 启动监听器 sidecar 进程
fn start_listener_sidecar(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = LISTENER.lock().unwrap();
    listener.start(app_handle)?;
    Ok(())
}

/// # 处理监听器事件
/// 可添加必要事件处理逻辑：
/// - 发送事件到前端
/// - 记录到数据库
/// - 触发特定操作等
/// 但是目前的架构设计上，事件处理将在 listener 中完成，而不需要在主线程处理
/// 主线程仅处理 horologion 的业务逻辑即可，核心监听交由 listener 完成
fn handle_listener_event(event_line: &str, _app_handle: &tauri::AppHandle) {
    // 解析事件数据并处理
    info!("Processing event: {}", event_line);
    
    // 如果需要发送事件到前端，可以使用：
    // app_handle.emit_all("listener-event", event_line).ok();
}

/// 停止监听器子进程
fn cleanup_listener_process() {
    info!("Cleaning up listener process...");
    let mut listener = LISTENER.lock().unwrap();
    listener.stop();
}
