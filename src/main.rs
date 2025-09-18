mod db;
mod getter;

use clap::Parser;
use getter::{get_available_translations, get_verse};
use std::error::Error;

#[derive(Parser)]
struct Cli {
    book: String,
    chapter: i32,
    verse: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let verses = get_verse(&args.book, args.chapter, args.verse)?;
    for verse in verses {
        println!("{}", verse);
    }
    Ok(())
}
