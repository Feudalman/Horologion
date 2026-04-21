//! Tauri server 层入口。
//!
//! 这个模块负责组织所有暴露给前端的 command、共享应用状态，以及数据库连接生命周期。
//! 业务接口按职责拆到子模块中，例如事件相关接口在 `events`，运行状态和设置相关接口在
//! `settings`，command 绑定集中放在 `router`。

pub mod events;
pub mod router;
pub mod settings;
pub mod windows;

use database::{
    api::init as init_database,
    db::{DatabaseManager, DatabaseTarget, RunMode},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri_plugin_shell::process::CommandChild;

/// Tauri server 层共享状态。
///
/// `ServerState` 被注册到 Tauri 的 managed state 中，供每个 command 通过
/// `tauri::State` 读取。这里持有数据库管理器，也记录 listener sidecar 的运行状态。
pub struct ServerState {
    /// src-db 提供的数据库连接管理器，server 层只通过它调用数据库 API。
    db: DatabaseManager,
    /// listener sidecar 是否仍被认为处于运行状态。
    listener_running: Arc<AtomicBool>,
    /// 保存 sidecar 子进程句柄，避免 setup 结束后句柄被 drop。
    listener_child: Mutex<Option<CommandChild>>,
}

impl ServerState {
    /// 从环境变量初始化数据库连接和 schema。
    ///
    /// Tauri 主进程和 listener sidecar 使用同一套 `src-db` 配置规则，因此这里也走
    /// `DatabaseManager::from_env`。初始化阶段会确保 schema 可用，避免前端第一次调用
    /// command 时才遇到缺表错误。
    pub fn from_env() -> Result<Self, String> {
        let db = DatabaseManager::from_env().map_err(|error| error.to_string())?;
        db.init().map_err(|error| error.to_string())?;
        db.with_connection(init_database)
            .map_err(|error| error.to_string())?;

        Ok(Self {
            db,
            listener_running: Arc::new(AtomicBool::new(false)),
            listener_child: Mutex::new(None),
        })
    }

    /// 返回数据库管理器，供 command 调用 src-db API。
    pub fn db(&self) -> &DatabaseManager {
        &self.db
    }

    /// 返回当前数据库运行模式。
    pub fn run_mode(&self) -> &RunMode {
        &self.db.config().mode
    }

    /// 返回当前数据库文件路径；内存数据库没有路径。
    pub fn database_path(&self) -> Option<String> {
        match &self.db.config().target {
            DatabaseTarget::Memory => None,
            DatabaseTarget::File(path) => Some(path.to_string_lossy().to_string()),
        }
    }

    /// 读取 listener sidecar 的运行状态。
    pub fn is_listener_running(&self) -> bool {
        self.listener_running.load(Ordering::SeqCst)
    }

    /// 更新 listener sidecar 的运行状态。
    pub fn set_listener_running(&self, running: bool) {
        self.listener_running.store(running, Ordering::SeqCst);
    }

    /// 返回可移动到异步任务中的运行状态句柄。
    pub fn listener_running_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.listener_running)
    }

    /// 保存 sidecar 子进程句柄，让 listener 跟随 Tauri 应用生命周期运行。
    pub fn set_listener_child(&self, child: CommandChild) {
        let mut guard = self.listener_child.lock().unwrap();
        *guard = Some(child);
    }
}
