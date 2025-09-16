use rusqlite::{Connection, Result};
use std::path::Path;

pub fn db_exists(db_path: &str) -> bool {
    Path::new(db_path).exists()
}

pub fn conn(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
}
