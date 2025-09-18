mod db;
mod getter;

use getter::{get_available_translations, get_verse};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let verses = get_verse("Genesis", 1, 1)?;
    for verse in verses {
        println!("{}", verse);
    }

    let translations = get_available_translations()?;
    for translation in translations {
        println!("{}", translation);
    }
    Ok(())
}
