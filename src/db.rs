use crate::errors::AppError;
use dirs::data_dir;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

/// Returns the path to the database file, ensuring the directory exists.
pub fn get_db_path() -> Result<PathBuf, AppError> {
    // Check for local bible.db for development
    let local_path = PathBuf::from("bible.db");
    if local_path.exists() {
        return Ok(local_path);
    }

    // Original logic for installed version
    let mut path =
        data_dir().ok_or_else(|| AppError::NotFound("a valid data directory".to_string()))?;
    path.push("bible-cli"); // App-specific folder

    // Create the directory if it doesn't exist
    fs::create_dir_all(&path)?;

    path.push("bible.db");
    Ok(path)
}

/// Opens a connection to the database.
pub fn conn() -> Result<Connection, AppError> {
    let db_path = get_db_path()?;
    let connection = Connection::open(db_path)?;
    Ok(connection)
}
