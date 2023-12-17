use crate::{
    app::{models::songs::SongFormats, utils::dirs::get_app_songs_dir},
    common::errors::AppError,
};
use reqwest::{Client, Url};
use std::{
    fs::{self, File},
    io::Write,
};

#[tauri::command]
pub async fn download_openlp_song(url: String) -> Result<String, AppError> {
    let file_url = match Url::parse(url.as_str()) {
        Ok(val) => Ok(val),
        Err(err) => Err(AppError::new(err.to_string().as_str())),
    }?;
    let client = Client::new();
    let url_parts: Vec<&str> = url.split("/").collect();

    let file_name = match url_parts.last() {
        Some(name) => Ok(format!("{}", name)),
        None => Err(AppError::new("Invalid file url")),
    }?;

    let openlp_song_dir = get_app_songs_dir(Some("openlp"))?;
    let file_path = openlp_song_dir.join(&file_name);

    let mut file = File::create(&file_path)?;

    let res = client.get(file_url).send().await?;
    let content = res.bytes().await?;
    file.write_all(&content)?;

    return Ok(format!("Succefully downloaded {}", file_name));
}

#[tauri::command]
pub fn get_downloaded_song_dbs(format: SongFormats) -> Result<Vec<String>, AppError> {
    let songs_dir = get_app_songs_dir(Some(format.to_string().to_lowercase().as_str()))?;
    let mut song_dbs: Vec<String> = Vec::new();
    let entries = fs::read_dir(&songs_dir)?;

    for entry in entries {
        song_dbs.push(entry?.file_name().to_string_lossy().to_string())
    }

    return Ok(song_dbs);
}
