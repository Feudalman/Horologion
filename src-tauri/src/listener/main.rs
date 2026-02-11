mod window;
mod monitor;

use monitor::EventListener;

/// 键鼠监听器入口
fn main() {
    // 初始化日志
    env_logger::init();

    // 创建并启动事件监听器
    let listener = EventListener::new();
    if let Err(error) = listener.start() {
        eprintln!("Failed to start listener: {}", error);
        std::process::exit(1);
    }
}
