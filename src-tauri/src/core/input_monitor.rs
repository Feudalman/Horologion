use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, BufReader};
use log::{info, error, warn};

static MONITORING: AtomicBool = AtomicBool::new(false);
static mut CHILD_PROCESS: Option<Child> = None;

pub fn start_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // 重复启动，不做处理
    if MONITORING.load(Ordering::Relaxed) {
        return Err("monitor is already running".into());
    }
    
    MONITORING.store(true, Ordering::Relaxed);
    info!("starting input monitoring process...");
    
    // 启动子进程
    tokio::spawn(async {
        if let Err(e) = run_input_listener().await {
            error!("Input listener process error: {}", e);
            MONITORING.store(false, Ordering::Relaxed);
        }
    });
    
    Ok(())
}

pub fn stop_monitoring() {
    MONITORING.store(false, Ordering::Relaxed);
    
    // 终止子进程
    unsafe {
        if let Some(mut child) = CHILD_PROCESS.take() {
            if let Err(e) = child.kill() {
                warn!("Failed to kill child process: {}", e);
            } else {
                info!("Child process terminated");
            }
        }
    }
    
    info!("monitor stopped");
}

async fn run_input_listener() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 获取当前可执行文件的路径
    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or("Failed to get exe directory")?;
    let listener_path = exe_dir.join("input_listener");
    
    info!("Starting input listener at: {:?}", listener_path);
    
    // 启动子进程
    let mut child = Command::new(&listener_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // 保存子进程引用以便后续终止
    unsafe {
        CHILD_PROCESS = Some(child);
        if let Some(ref mut child) = CHILD_PROCESS {
            let stdout = child.stdout.take().ok_or("Failed to get stdout")?;
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            
            info!("Input listener process started, reading events...");
            
            // 读取子进程输出
            while MONITORING.load(Ordering::Relaxed) {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        process_event_line(&line);
                    }
                    Ok(None) => {
                        info!("Input listener process ended");
                        break;
                    }
                    Err(e) => {
                        error!("Error reading from input listener: {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    MONITORING.store(false, Ordering::Relaxed);
    Ok(())
}

fn process_event_line(line: &str) {
    // 解析事件数据并处理
    if line.starts_with("KeyPress:") {
        let key_data = &line[9..];
        info!("keyboard pressed: {}", key_data);
        // 这里可以添加持久化存储逻辑
    } else if line.starts_with("KeyRelease:") {
        let key_data = &line[11..];
        info!("keyboard released: {}", key_data);
        // 这里可以添加持久化存储逻辑
    } else if line.starts_with("ButtonPress:") {
        let button_data = &line[12..];
        info!("mouse pressed: {}", button_data);
        // 这里可以添加持久化存储逻辑
    } else if line.starts_with("ButtonRelease:") {
        let button_data = &line[14..];
        info!("mouse released: {}", button_data);
        // 这里可以添加持久化存储逻辑
    } else if line.starts_with("Wheel:") {
        let wheel_data = &line[6..];
        info!("mouse scroll: {}", wheel_data);
        // 这里可以添加持久化存储逻辑
    }
}
