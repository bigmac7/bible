mod config;
mod db;
mod downloader;
mod errors;
mod getter;

use clap::{Parser, Subcommand};
use config::get_default_translation;
use downloader::add_translations;
use errors::AppError;
use getter::{get_available_translations, get_chapter, get_verse};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A simple CLI for downloading and reading the Bible.\n\nENVIRONMENT:\n    BIBLE_DEFAULT_TRANSLATION    Sets the default translation to be installed on first run. Defaults to \"KJV\".",
    arg_required_else_help = true
)]
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
        #[arg(short, long)]
        translation: Option<String>,
    },

    /// Add one or more new translations by downloading them
    #[command(long_about = include_str!(concat!(env!("OUT_DIR"), "/remote_translations.txt")))]
    Add {
        /// A list of translation abbreviations to add (e.g., ASV BBE)
        #[arg(required = true)]
        translations: Vec<String>,
    },

    /// Configure default settings
    Config {
        /// Set the default translation
        translation: Option<String>,
    },

    /// List all available translations in the database
    List,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let mut config = config::load_config()?;
    let default_translation = get_default_translation()?;
    let available_translations = get_available_translations().unwrap_or_default();

    if !available_translations.contains(&default_translation) {
        println!(
            "Default translation ({}) not found. Downloading and installing...",
            default_translation
        );
        add_translations(&vec![default_translation.clone()])?;
        config.default_translation = Some(default_translation.clone());
        config::save_config(&config)?;
        println!("Default translation installed.");
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Get {
            book,
            chapter,
            verse,
            translation,
        } => {
            let translation_to_use = match translation {
                Some(t) => t.clone(),
                None => get_default_translation()?,
            };

            if let Some(verse) = verse {
                let verses = get_verse(&translation_to_use, book, *chapter, *verse)?;
                println!("{} {}:{} ({})", book, chapter, verse, translation_to_use);
                for v in verses {
                    println!("{}", v);
                }
            } else {
                let verses = get_chapter(&translation_to_use, book, *chapter)?;
                println!("{} {} ({})", book, chapter, translation_to_use);
                for v in verses {
                    println!("{}", v);
                }
            }
        }
        Commands::Add { translations } => {
            add_translations(translations)?;
            if let Some(new_default) = translations.last() {
                let mut config = config::load_config()?;
                config.default_translation = Some(new_default.clone());
                config::save_config(&config)?;
                println!("Set '{}' as default translation.", new_default);
            }
        }
        Commands::Config { translation } => {
            let mut config = config::load_config()?;
            if let Some(new_default) = translation {
                config.default_translation = Some(new_default.clone());
                config::save_config(&config)?;
                println!("Set '{}' as default translation.", new_default);
            } else {
                let default = get_default_translation()?;
                println!("Current default translation: {}", default);
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
