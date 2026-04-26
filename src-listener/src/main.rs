mod monitor;
mod permissions;
mod window;

use database::db::{path::find_project_root, RunMode};
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use monitor::EventListener;
use std::path::PathBuf;

const LISTENER_LOG_BASENAME: &str = "listener";
const LOG_ROTATE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
const LOG_RETENTION_DAYS: usize = 30;

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
    let log_spec = default_log_spec();
    let log_directory = match log_directory() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Failed to resolve listener log directory: {}", error);
            init_fallback_logger(&log_spec);
            return;
        }
    };

    if let Err(error) = std::fs::create_dir_all(&log_directory) {
        eprintln!("Failed to create listener log directory: {}", error);
        init_fallback_logger(&log_spec);
        return;
    }

    let logger = Logger::try_with_str(&log_spec)
        .map(|logger| {
            logger
                .log_to_file(
                    FileSpec::default()
                        .directory(log_directory.clone())
                        .basename(LISTENER_LOG_BASENAME)
                        .suffix("log"),
                )
                .duplicate_to_stderr(Duplicate::All)
                .rotate(
                    Criterion::AgeOrSize(Age::Day, LOG_ROTATE_SIZE_BYTES),
                    Naming::Timestamps,
                    Cleanup::KeepForDays(LOG_RETENTION_DAYS),
                )
                .append()
        })
        .and_then(|logger| logger.start());

    if let Err(error) = logger {
        eprintln!("Failed to initialize listener logger: {}", error);
        init_fallback_logger(&log_spec);
        return;
    }

    log::info!(
        "Listener logs directory: {} (daily rotation, {} bytes max per file, retain {} days)",
        log_directory.display(),
        LOG_ROTATE_SIZE_BYTES,
        LOG_RETENTION_DAYS
    );
}

fn default_log_spec() -> String {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        std::env::set_var("RUST_LOG", &log_level);
    }

    std::env::var("RUST_LOG")
        .or_else(|_| std::env::var("LOG_LEVEL"))
        .unwrap_or_else(|_| "info".to_string())
}

fn init_fallback_logger(log_spec: &str) {
    let _ = Logger::try_with_str(log_spec).and_then(|logger| logger.log_to_stderr().start());
}

fn log_directory() -> Result<PathBuf, String> {
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

    Ok(log_dir)
}
