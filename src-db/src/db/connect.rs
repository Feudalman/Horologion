//! # db - connect
//!
//! 该模块负责数据库的连接

use duckdb::{Connection, Error as DuckError};

pub fn connect() -> Result<Connection, DuckError> {
    let conn = Connection::open("/Users/zirui/projects/arui/Horologion/playground/horologion.db");
    conn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let conn = connect();
        assert!(conn.is_ok());
    }
}
