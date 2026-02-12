mod window;
mod monitor;

use monitor::EventListener;

/// 键鼠监听器入口
fn main() {
    // 加载 .env 配置文件
    dotenvy::dotenv().ok();
    
    // 检查是否手动设置了日志级别，如果没有，则从配置文件读取，默认为 info
    if std::env::var("RUST_LOG").is_err() {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        std::env::set_var("RUST_LOG", log_level);
    }
    // 初始化日志
    env_logger::init();

    // 创建并启动事件监听器
    let listener = EventListener::new();
    if let Err(error) = listener.start() {
        eprintln!("Failed to start listener: {}", error);
        std::process::exit(1);
    }
}
