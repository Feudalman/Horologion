use crate::commands;
use log::{error, info, warn};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;

pub fn init_and_run() {
    // 检查是否手动设置了日志级别，如果没有，则设置为 info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // 初始化日志
    env_logger::init();

    // 启动 Tokio 运行时来管理异步任务
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    // 启动监听器子进程
    let listener_handle = rt.spawn(async {
        if let Err(e) = start_listener_process().await {
            error!("Failed to start listener process: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |_app| {
            // 在应用关闭时清理子进程
            std::thread::spawn(move || {
                rt.block_on(async {
                    // 等待监听器任务完成或应用退出
                    let _ = listener_handle.await;
                });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn start_listener_process() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting listener sidecar process...");
    
    // 获取当前可执行文件的路径
    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or("Failed to get executable directory")?;
    
    // 构建监听器可执行文件路径
    let listener_exe = if cfg!(windows) {
        exe_dir.join("listener.exe")
    } else {
        exe_dir.join("listener")
    };

    info!("Listener executable path: {:?}", listener_exe);

    // 启动子进程
    let mut child = TokioCommand::new(&listener_exe)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    info!("Listener process started with PID: {:?}", child.id());

    // 读取子进程的输出
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        
        tokio::spawn(async move {
            while let Ok(Some(line)) = lines.next_line().await {
                info!("Listener output: {}", line);
                // 这里可以处理从监听器接收到的事件数据
                handle_listener_event(&line).await;
            }
        });
    }

    // 读取子进程的错误输出
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        
        tokio::spawn(async move {
            while let Ok(Some(line)) = lines.next_line().await {
                warn!("Listener stderr: {}", line);
            }
        });
    }

    // 等待子进程结束
    let status = child.wait().await?;
    info!("Listener process exited with status: {}", status);

    Ok(())
}

async fn handle_listener_event(event_line: &str) {
    // 解析事件数据并处理
    // 例如：KeyPress:Key(A) -> 解析为键盘按下事件
    info!("Processing event: {}", event_line);
    
    // 这里可以添加事件处理逻辑，比如：
    // - 发送事件到前端
    // - 记录到数据库
    // - 触发特定操作等
}
