use rusqlite::{Connection, Result};
use std::path::Path;

pub fn db_exists(db_path: &str) -> bool {
    Path::new(db_path).exists()
}

pub fn conn() -> Result<Connection> {
    Connection::open("bible.db")
}
