#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub chapter_count: u32,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub book: Book,
    pub number: u32,
    pub verse_count: u32,
}

#[derive(Debug)]
pub struct Verse {
    pub book: Book,
    pub chapter: Chapter,
    pub verse: u32,
    pub text: String,
}

impl Verse {
    pub fn new(book: Book, chapter: Chapter, verse: u32, text: String) -> Self {
        Verse {
            book,
            chapter,
            verse,
            text,
        }
    }
}

impl Chapter {
    pub fn new(book: Book, number: u32, verse_count: u32) -> Self {
        Chapter {
            book,
            number,
            verse_count,
        }
    }
}

impl Book {
    pub fn new(name: String, chapter_count: u32) -> Self {
        Book {
            name,
            chapter_count,
        }
    }
}
