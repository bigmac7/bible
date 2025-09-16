use crate::db::conn;
use std::error::Error;

pub fn get_verse(book: &str, chapter: &str, verse: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let con = conn("KJV.db")?;

    let mut stmt = match con
        .prepare("SELECT verse FROM kjv_verses WHERE book = ? AND chapter = ? AND verse = ?")
    {
        Ok(stmt) => stmt,
        Err(err) => {
            eprintln!("Error preparing statement: {}", err);
            return Err(err.into());
        }
    };

    let mut rows = match stmt.query([book, chapter, verse]) {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error querying database: {}", err);
            return Err(err.into());
        }
    };

    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        data.push(row.get(0)?);
    }

    Ok(data)
}
