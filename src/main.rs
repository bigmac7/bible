mod db;
mod downloader;
mod getter;

use clap::{Parser, Subcommand};
use downloader::add_translations;
use getter::{get_available_translations, get_chapter, get_verse};
use std::error::Error;

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
        verse: Option<i32>,

        /// The translation to use (e.g., KJV, ASV)
        #[arg(short, long, default_value = "KJV")]
        translation: String,
    },

    /// Add one or more new translations by downloading them
    #[command(long_about = include_str!(concat!(env!("OUT_DIR"), "/remote_translations.txt")))]
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
            if let Some(verse) = verse {
                let verses = get_verse(translation, book, *chapter, *verse)?;
                println!("{} {}:{} ({})", book, chapter, verse, translation);
                for v in verses {
                    println!("{}", v);
                }
            } else {
                let verses = get_chapter(translation, book, *chapter)?;
                println!("{} {} ({})", book, chapter, translation);
                for v in verses {
                    println!("{}", v);
                }
            }
        }
        Commands::Add { translations } => {
            add_translations(translations)?;
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
