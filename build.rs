use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("remote_translations.txt");
    let mut f = File::create(&dest_path)?;

    let url = "https://raw.githubusercontent.com/scrollmapper/bible_databases/2025/docs/main_readme/translation_list.md";
    let body = reqwest::blocking::get(url)?.text()?;
    let mut translations = String::new();
    translations.push_str("Available translations for download:\n");
    for line in body.lines() {
        if line.starts_with("- **") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let name_part = parts[0].replace("- **", "").replace("**", "");
                let desc_part = parts[1].trim();
                translations.push_str(&format!("- {}: {}\n", name_part, desc_part));
            }
        }
    }

    f.write_all(translations.as_bytes())?;
    Ok(())
}
