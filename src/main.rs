mod db;
mod getter;
mod structs;

use getter::get_verse;

fn main() {
    match get_verse("Genesis", "1", "1") {
        Ok(verses) => {
            for verse in verses {
                println!("{}", verse);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
