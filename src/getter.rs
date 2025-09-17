use crate::db::conn;
use std::error::Error;

fn get_book_id(book: &str) -> Result<i32, Box<dyn Error>> {
    let con = conn("KJV.db")?;
    let mut stmt = con.prepare("SELECT id FROM kjv_books WHERE name = ?")?;
    let book_id = stmt.query_row([book], |row| row.get(0))?;
    Ok(book_id)
}

pub fn get_verse(book: &str, chapter: i32, verse: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let con = conn("KJV.db")?;
    let book_id = get_book_id(book)?;
    let mut stmt =
        con.prepare("SELECT text FROM kjv_verses WHERE book_id = ? AND chapter = ? AND verse = ?")?;
    let rows = stmt.query_map([book_id, chapter, verse], |row| row.get(0))?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}
