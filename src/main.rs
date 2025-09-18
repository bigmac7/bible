mod db;
mod getter;

use clap::{Parser, Subcommand};
use getter::{get_available_translations, get_verse};
use std::error::Error;
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a bible verse by book, chapter, and verse number
    Get {
        book: String,
        chapter: i32,
        verse: i32,

        /// The translation to use (e.g., KJV, ASV)
        #[arg(short, long, default_value = "KJV")]
        translation: String,
    },

    /// Add one or more new translations by downloading them
    Add {
        /// A list of translation abbreviations to add (e.g., ASV BBE)
        #[arg(required = true)]
        translations: Vec<String>,
    },

    /// List all available translations in the database
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get {
            book,
            chapter,
            verse,
            translation,
        } => {
            let verses = get_verse(translation, book, *chapter, *verse)?;
            println!("{} {}:{} ({})", book, chapter, verse, translation);
            for v in verses {
                println!("{}", v);
            }
        }
        Commands::Add { translations } => {
            println!(
                "Attempting to add translations: {}...",
                translations.join(", ")
            );
            let mut cmd = Command::new("./get_bible_sqlite_db.sh")
                .args(translations)
                .spawn()?;

            let status = cmd.wait()?;
            if status.success() {
                println!("Script finished successfully.");
            } else {
                eprintln!("Script failed with status: {}", status);
            }
        }
        Commands::List => {
            println!("Available translations:");
            let translations = get_available_translations()?;
            for t in translations {
                println!("- {}", t);
            }
        }
    }

    Ok(())
}
