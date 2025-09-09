use std::env;

mod parser;
pub mod structs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <book> <chapter> <verse>", args[0]);
        return;
    }

    let bible_data = parser::parse_bible_text();

    let book = &args[1];
    let chapter = &args[2];
    let verse = &args[3];
    let reference = format!("{}:{}:{}", book, chapter, verse);

    match bible_data.get(&reference) {
        Some(verse_data) => println!("{}", verse_data.text),
        None => eprintln!("Verse not found for reference: {}", reference),
    }
}
