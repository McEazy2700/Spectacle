use crate::{app::utils::dirs::get_bible_dir, common::errors::AppError};
use std::fs;

pub mod download;
pub mod queries;

#[tauri::command]
pub fn get_downloaded_bible_versions() -> Result<Vec<String>, AppError> {
    let bible_dir = get_bible_dir()?;
    let mut bible_versions: Vec<String> = Vec::new();
    let entries = fs::read_dir(&bible_dir)?;

    for entry in entries {
        let file_name = entry?.file_name();
        let file_name_str = file_name.to_string_lossy();
        let name_parts: Vec<&str> = file_name_str.split(".").collect();
        if let Some(name) = name_parts.first() {
            bible_versions.push(name.to_string());
        }
    }

    Ok(bible_versions)
}
