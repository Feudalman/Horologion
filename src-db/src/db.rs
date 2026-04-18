//! # db 模块
//!
//! 该模块负责数据库的各种处理，和具体业务逻辑无关

pub mod config;
pub mod connect;
pub mod path;

pub use config::{DatabaseConfig, DatabaseSource, DatabaseTarget};
pub use connect::{connect, connect_from_env, DatabaseManager};
pub use path::RunMode;
