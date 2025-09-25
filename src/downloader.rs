use crate::db;
use crate::errors::AppError;
use crate::getter::get_available_translations;
use rusqlite::{Connection, Result as RusqliteResult};
use std::fs::{self, File};
use std::io;
use std::path::Path;

const BASE_URL: &str =
    "https://raw.githubusercontent.com/scrollmapper/bible_databases/master/formats/sqlite/";

pub fn add_translations(translations: &Vec<String>) -> Result<(), AppError> {
    if translations.is_empty() {
        return Ok(()); // Nothing to do
    }

    let db_path = db::get_db_path()?;
    let existing_translations = if db_path.exists() {
        get_available_translations()?
    } else {
        Vec::new()
    };

    let mut translations_to_process = Vec::new();
    for t in translations {
        if existing_translations
            .iter()
            .any(|et| et.eq_ignore_ascii_case(t))
        {
            println!("ℹ️  Translation '{}' already exists. Skipping.", t);
        } else {
            translations_to_process.push(t.clone());
        }
    }

    if translations_to_process.is_empty() {
        println!("🎉  All requested translations are already in the database.");
        return Ok(());
    }

    let mut first_translation_processed = false;
    if !db_path.exists() {
        println!(
            "ℹ️  '{}' not found. Creating it from: {}",
            db_path.display(),
            &translations_to_process[0]
        );
        let first_translation = &translations_to_process[0];
        let temp_path = download_translation(first_translation)?;
        fs::rename(&temp_path, &db_path)?;
        fs::remove_file(&temp_path).unwrap_or_default(); // Clean up temp file if rename fails across devices
        first_translation_processed = true;
    }

    let translations_to_merge: Vec<&String> = if first_translation_processed {
        translations_to_process.iter().skip(1).collect()
    } else {
        translations_to_process.iter().collect()
    };

    for translation in translations_to_merge {
        if translation.to_lowercase() == "bible.db" {
            continue;
        } // Just in case

        let temp_db_path = download_translation(translation)?;

        println!("Merging {} into {}...", translation, db_path.display());
        merge_databases(&db_path, &temp_db_path)?;

        fs::remove_file(temp_db_path)?;
    }

    println!("🎉  Finished.");
    Ok(())
}

fn download_translation(translation: &str) -> Result<std::path::PathBuf, AppError> {
    let url = format!("{}{}.db", BASE_URL, translation);
    println!("Downloading {}...", url);
    let response = reqwest::blocking::get(&url)?.error_for_status()?;

    let temp_path = std::env::temp_dir().join(format!("{}.db", translation));
    let mut dest = File::create(&temp_path)?;
    let content = response.bytes()?;
    io::copy(&mut content.as_ref(), &mut dest)?;
    Ok(temp_path)
}

fn merge_databases(main_db_path: &Path, other_db_path: &Path) -> Result<(), AppError> {
    let mut main_conn = Connection::open(main_db_path)?;
    let other_db_path_str = other_db_path.to_str().ok_or_else(|| {
        AppError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid temporary database path",
        ))
    })?;

    main_conn.execute("ATTACH DATABASE ?1 AS toMerge;", [other_db_path_str])?;

    let tx = main_conn.transaction()?;

    let (main_tables, other_tables) = {
        let mut stmt_main = tx.prepare(
            "SELECT name FROM main.sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )?;
        let main_tables = stmt_main
            .query_map([], |row| row.get(0))?
            .collect::<RusqliteResult<Vec<String>>>()?;

        let mut stmt_other = tx.prepare(
            "SELECT name FROM toMerge.sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )?;
        let other_tables = stmt_other
            .query_map([], |row| row.get(0))?
            .collect::<RusqliteResult<Vec<String>>>()?;
        (main_tables, other_tables)
    };

    for table in &other_tables {
        let sql = if main_tables.contains(table) {
            format!(
                "INSERT OR IGNORE INTO main.{} SELECT * FROM toMerge.{};",
                table, table
            )
        } else {
            format!(
                "CREATE TABLE main.{} AS SELECT * FROM toMerge.{};",
                table, table
            )
        };
        tx.execute(&sql, [])?;
    }

    tx.commit()?;

    main_conn.execute("DETACH DATABASE toMerge;", [])?;

    Ok(())
}
