use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;

/// 监听器管理器
struct ListenerManager {
    child: Option<Child>,
}

/// 实现监听器管理器
impl ListenerManager {
    fn new() -> Self {
        Self { child: None }
    }

    /// 启动监听器子进程
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // 检查是否已经启动，不允许重复启动
        if self.child.is_some() {
            warn!("Listener process is already running");
            return Ok(());
        }

        info!("Starting listener sidecar process...");

        // 获取当前可执行文件的路径
        let current_exe = std::env::current_exe()?;
        let exe_dir = current_exe
            .parent()
            .ok_or("Failed to get executable directory")?;

        // 构建监听器可执行文件路径
        let listener_exe = if cfg!(windows) {
            exe_dir.join("listener.exe")
        } else {
            exe_dir.join("listener")
        };

        info!("Listener executable path: {:?}", listener_exe);

        // 启动子进程
        let child = Command::new(&listener_exe)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        info!("Listener process started with PID: {:?}", child.id());

        self.child = Some(child);
        Ok(())
    }

    /// 停止监听器子进程
    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            match child.kill() {
                Ok(_) => {
                    info!("Listener process terminated successfully");
                    let _ = child.wait(); // 等待进程完全退出
                }
                Err(e) => error!("Failed to terminate listener process: {}", e),
            }
        }
    }

    /// 取出子进程
    /// 将移除 child 并返回一个新的子进程引用，原先的 child 将为 None
    fn take_child(&mut self) -> Option<Child> {
        self.child.take()
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

    // 启动监听器子进程
    if let Err(e) = start_listener_process() {
        error!("Failed to start listener process: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // 注册应用退出时的清理函数
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

/// 启动监听器子进程
fn start_listener_process() -> Result<(), Box<dyn Error>> {
    let mut listener = LISTENER.lock().unwrap();
    listener.start()?;

    // 取出子进程以便在新线程中处理输出
    if let Some(mut child) = listener.take_child() {
        thread::spawn(move || {
            // 读取标准输出
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                thread::spawn(move || {
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                info!("Listener output: {}", line);
                                handle_listener_event(&line);
                            }
                            Err(e) => {
                                error!("Error reading listener stdout: {}", e);
                                break;
                            }
                        }
                    }
                });
            }

            // 读取错误输出
            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                thread::spawn(move || {
                    for line in reader.lines() {
                        match line {
                            Ok(line) => warn!("Listener stderr: {}", line),
                            Err(e) => {
                                error!("Error reading listener stderr: {}", e);
                                break;
                            }
                        }
                    }
                });
            }

            // 将子进程重新放回管理器
            let mut listener = LISTENER.lock().unwrap();
            listener.child = Some(child);
        });
    }

    Ok(())
}

/// # 处理监听器事件
/// 可添加必要事件处理逻辑：
/// - 发送事件到前端
/// - 记录到数据库
/// - 触发特定操作等
/// 但是目前的架构设计上，事件处理将在 listener 中完成，而不需要在主线程处理
/// 主线程仅处理 horologion 的业务逻辑即可，核心监听交由 listener 完成
fn handle_listener_event(event_line: &str) {
    // 解析事件数据并处理
    info!("Processing event: {}", event_line);
}

/// 停止监听器子进程
fn cleanup_listener_process() {
    info!("Cleaning up listener process...");
    let mut listener = LISTENER.lock().unwrap();
    listener.stop();
}
