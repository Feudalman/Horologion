mod monitor;
mod permissions;
mod window;

use config::{app, env, logging, paths};
use database::db::{path::find_project_root, RunMode};
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use monitor::EventListener;
use std::path::PathBuf;

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
                        .basename(logging::LISTENER_BASENAME)
                        .suffix("log"),
                )
                .duplicate_to_stderr(Duplicate::All)
                .rotate(
                    Criterion::AgeOrSize(Age::Day, logging::ROTATE_SIZE_BYTES),
                    Naming::Timestamps,
                    Cleanup::KeepForDays(logging::RETENTION_DAYS),
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
        logging::ROTATE_SIZE_BYTES,
        logging::RETENTION_DAYS
    );
}

fn default_log_spec() -> String {
    if std::env::var(env::RUST_LOG).is_err() {
        let log_level = std::env::var(env::LOG_LEVEL).unwrap_or_else(|_| "info".to_string());
        std::env::set_var(env::RUST_LOG, &log_level);
    }

    std::env::var(env::RUST_LOG)
        .or_else(|_| std::env::var(env::LOG_LEVEL))
        .unwrap_or_else(|_| "info".to_string())
}

fn init_fallback_logger(log_spec: &str) {
    let _ = Logger::try_with_str(log_spec).and_then(|logger| logger.log_to_stderr().start());
}

fn log_directory() -> Result<PathBuf, String> {
    let log_dir = match RunMode::from_env() {
        RunMode::Test => std::env::temp_dir()
            .join(app::NAME)
            .join(logging::DIRECTORY_NAME),
        RunMode::Development => find_project_root()
            .map_err(|error| error.to_string())?
            .join(paths::PLAYGROUND_DIR)
            .join(logging::DIRECTORY_NAME),
        RunMode::Production => dirs::data_dir()
            .ok_or_else(|| "Failed to resolve system data directory".to_string())?
            .join(app::NAME)
            .join(logging::DIRECTORY_NAME),
    };

    Ok(log_dir)
}
