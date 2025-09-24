use crate::db::conn;
use crate::errors::AppError;
use rusqlite::params;

fn get_book_id(translation: &str, book: &str) -> Result<i32, AppError> {
    let con = conn()?;
    let table_name = format!("{}_books", translation.to_uppercase());
    let sql = format!("SELECT id FROM {} WHERE LOWER(name) = LOWER(?)", table_name);
    let mut stmt = con.prepare(&sql)?;
    let book_id_result = stmt.query_row([book], |row| row.get(0));
    match book_id_result {
        Ok(id) => Ok(id),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err(AppError::NotFound(format!("book '{}'", book)))
        }
        Err(e) => Err(e.into()),
    }
}

pub fn get_verse(
    translation: &str,
    book: &str,
    chapter: i32,
    verse: i32,
) -> Result<Vec<String>, AppError> {
    let con = conn()?;
    let book_id = get_book_id(translation, book)?;
    let table_name = format!("{}_verses", translation.to_uppercase());
    let sql = format!(
        "SELECT text FROM {} WHERE book_id = ? AND chapter = ? AND verse = ?",
        table_name
    );
    let mut stmt = con.prepare(&sql)?;
    let rows = stmt.query_map(params![book_id, chapter, verse], |row| row.get(0))?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}

pub fn get_available_translations() -> Result<Vec<String>, AppError> {
    let con = conn()?;
    let mut stmt =
        con.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%_verses'")?;
    let rows = stmt.query_map([], |row| {
        let table_name: String = row.get(0)?;
        Ok(table_name.replace("_verses", ""))
    })?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}

pub fn get_chapter(translation: &str, book: &str, chapter: i32) -> Result<Vec<String>, AppError> {
    let con = conn()?;
    let book_id = get_book_id(translation, book)?;
    let table_name = format!("{}_verses", translation.to_uppercase());
    let sql = format!(
        "SELECT text FROM {} WHERE book_id = ? AND chapter = ?",
        table_name
    );
    let mut stmt = con.prepare(&sql)?;
    let rows = stmt.query_map(params![book_id, chapter], |row| row.get(0))?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}
