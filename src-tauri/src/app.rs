use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;

struct ListenerManager {
    child: Option<Child>,
}

impl ListenerManager {
    fn new() -> Self {
        Self { child: None }
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn take_child(&mut self) -> Option<Child> {
        self.child.take()
    }
}

static LISTENER: Lazy<Mutex<ListenerManager>> = Lazy::new(|| Mutex::new(ListenerManager::new()));

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

fn start_listener_process() -> Result<(), Box<dyn std::error::Error>> {
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

fn handle_listener_event(event_line: &str) {
    // 解析事件数据并处理
    // 例如：KeyPress:Key(A) -> 解析为键盘按下事件
    info!("Processing event: {}", event_line);

    // 这里可以添加事件处理逻辑，比如：
    // - 发送事件到前端
    // - 记录到数据库
    // - 触发特定操作等
}

fn cleanup_listener_process() {
    info!("Cleaning up listener process...");
    let mut listener = LISTENER.lock().unwrap();
    listener.stop();
}
