use crate::structs::{Book, Chapter, Verse};
use std::collections::HashMap;
use std::fs;

pub fn parse_bible_text() -> HashMap<String, Verse> {
    let content = fs::read_to_string("NKJV.txt").expect("Something went wrong reading the file");
    let mut bible_data = HashMap::new();
    let mut current_book: Option<Book> = None;
    let mut current_chapter: Option<Chapter> = None;
    let mut current_verse = 0;
    let mut verse_text = String::new();

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        if line.starts_with('\u{000C}') && !line.chars().any(char::is_numeric) {
            let book_name = line.trim_start_matches('\u{000C}').trim().to_string();
            current_book = Some(Book::new(book_name, 0));
            current_chapter = None;
            continue;
        }

        if let Some(book) = &current_book {
            if trimmed_line
                .to_uppercase()
                .starts_with(&book.name.to_uppercase())
            {
                continue;
            }
        }

        if let Ok(number) = trimmed_line.parse::<u32>() {
            if current_chapter.is_none() {
                if let Some(book) = &current_book {
                    current_chapter = Some(Chapter::new(book.clone(), number, 0));
                    continue;
                }
            }

            if current_verse != 0 {
                if let (Some(book), Some(chapter)) = (&current_book, &current_chapter) {
                    let reference = format!("{}:{}:{}", book.name, chapter.number, current_verse);
                    let verse = Verse::new(
                        book.clone(),
                        chapter.clone(),
                        current_verse,
                        verse_text.trim().to_string(),
                    );
                    bible_data.insert(reference, verse);
                    verse_text.clear();
                }
            }

            let is_chapter_num = number > 0 && number < 151;
            let is_verse_num_in_header = line.len() < 5;
            if is_chapter_num && is_verse_num_in_header {
                if let Some(book) = &current_book {
                    current_chapter = Some(Chapter::new(book.clone(), number, 0));
                    current_verse = 0;
                    continue;
                }
            }
        }

        let mut parts = trimmed_line.splitn(2, ' ');
        if let Some(first_part) = parts.next() {
            if let Ok(verse_num) = first_part.parse::<u32>() {
                if current_verse != 0 {
                    if let (Some(book), Some(chapter)) = (&current_book, &current_chapter) {
                        let reference =
                            format!("{}:{}:{}", book.name, chapter.number, current_verse);
                        let verse = Verse::new(
                            book.clone(),
                            chapter.clone(),
                            current_verse,
                            verse_text.trim().to_string(),
                        );
                        bible_data.insert(reference, verse);
                    }
                }
                current_verse = verse_num;
                verse_text = parts.next().unwrap_or("").to_string();
            } else {
                verse_text.push_str(" ");
                verse_text.push_str(trimmed_line);
            }
        }
    }

    if !verse_text.is_empty() {
        if let (Some(book), Some(chapter)) = (&current_book, &current_chapter) {
            let reference = format!("{}:{}:{}", book.name, chapter.number, current_verse);
            let verse = Verse::new(
                book.clone(),
                chapter.clone(),
                current_verse,
                verse_text.trim().to_string(),
            );
            bible_data.insert(reference, verse);
        }
    }

    bible_data
}
