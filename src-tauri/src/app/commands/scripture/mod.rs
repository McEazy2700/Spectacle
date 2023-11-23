use std::{
    fs::{self, File},
    io,
};

use reqwest::Url;

use crate::{app::utils::dirs::get_bible_dir, common::errors::AppError};

pub mod download;

#[tauri::command]
pub async fn download_kjv_bible() -> anyhow::Result<String, AppError> {
    let bible_url = match Url::parse(
        "http://simoncozens.github.io/open-source-bible-data/cooked/sqlite/kjv.db",
    ) {
        Ok(val) => Ok(val),
        Err(err) => Err(AppError::new(err.to_string().as_str())),
    }?;
    let destination = get_bible_dir()?.join("kjv.db");
    let mut file = File::create(&destination)?;

    let response = reqwest::get(bible_url).await?;
    if !response.status().is_success() {
        return Err(AppError::new("Could not download bible"));
    }
    let body = response.bytes().await?;
    io::copy(&mut body.as_ref(), &mut file)?;
    Ok(String::from("Success: KJV bible downloaded successfully"))
}

#[tauri::command]
pub fn get_downloaded_bible_versions() -> anyhow::Result<Vec<String>, AppError> {
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
