//! 数据模型与表结构定义。
//!
//! 这个模块负责 `src-db` 持久化的数据形状。
//! 底层数据库连接、配置和路径仍然放在 `db` 模块中；表结构和业务模型类型放在这里。

pub mod listener;
pub mod schema;

pub use listener::{InputEvent, InputEventKind, ObservedWindow};
pub use schema::{
    init_schema, SCHEMA_SQL, SCHEMA_VERSION, TABLE_INPUT_EVENTS, TABLE_OBSERVED_WINDOWS,
};
