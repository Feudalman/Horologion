mod monitor;
mod permissions;
mod window;

use database::db::{path::find_project_root, RunMode};
use log::LevelFilter;
use monitor::EventListener;
use std::fs::OpenOptions;
use std::path::PathBuf;

const LISTENER_LOG_FILE: &str = "listener.log";

/// 键鼠监听器入口
fn main() {
    // 加载 .env 配置文件
    dotenvy::dotenv().ok();

    init_log();

    // 创建并启动事件监听器
    let listener = match EventListener::new() {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Failed to initialize listener: {}", error);
            std::process::exit(1);
        }
    };

    if let Err(error) = listener.start() {
        eprintln!("Failed to start listener: {}", error);
        std::process::exit(1);
    }
}

fn init_log() {
    let log_level = default_log_level();
    let log_file_path = match log_file_path(LISTENER_LOG_FILE) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Failed to resolve listener log path: {}", error);
            init_fallback_logger(log_level);
            return;
        }
    };

    if let Some(parent) = log_file_path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create listener log directory: {}", error);
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
            eprintln!(
                "Failed to open listener log file {:?}: {}",
                log_file_path, error
            );
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
        eprintln!("Failed to initialize listener logger: {}", error);
        init_fallback_logger(log_level);
        return;
    }

    log::info!("Listener log file: {}", log_file_path.display());
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
