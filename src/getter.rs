use crate::db::conn;
use std::error::Error;

fn get_book_id(book: &str) -> Result<i32, Box<dyn Error>> {
    let con = conn()?;
    let mut stmt = con.prepare("SELECT id FROM kjv_books WHERE name = ?")?;
    let book_id = stmt.query_row([book], |row| row.get(0))?;
    Ok(book_id)
}

pub fn get_verse(book: &str, chapter: i32, verse: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let con = conn()?;
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

pub fn get_available_translations() -> Result<Vec<String>, Box<dyn Error>> {
    let con = conn()?;
    let mut stmt = con.prepare("SELECT * FROM translations")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}

pub fn get_chapter(book: &str, chapter: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let con = conn()?;
    let book_id = get_book_id(book)?;
    let mut stmt = con.prepare("SELECT text FROM kjv_verses WHERE book_id = ? AND chapter = ?")?;
    let rows = stmt.query_map([book_id, chapter], |row| row.get(0))?;
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }
    Ok(data)
}
