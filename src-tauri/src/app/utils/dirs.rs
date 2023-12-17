use std::{fs, path::PathBuf};

use crate::{app::config::constants::APP_NAME, common::errors::AppError};

pub fn get_app_data_dir() -> Result<PathBuf, AppError> {
    let config_dir = match dirs::data_dir() {
        Some(dir) => Ok(dir),
        None => Err(AppError::new("Couldn't determine config directory")),
    }?;
    let app_config_dir = config_dir.join(APP_NAME);
    if !app_config_dir.exists() {
        fs::create_dir(&app_config_dir)?
    }
    Ok(app_config_dir)
}

pub fn get_bible_dir() -> Result<PathBuf, AppError> {
    let data_dir = get_app_data_dir()?;
    let bible_dir = data_dir.join("bible");
    if !bible_dir.is_dir() {
        fs::create_dir(&bible_dir)?;
    }
    return Ok(bible_dir);
}

pub fn get_app_temp_dir() -> Result<PathBuf, AppError> {
    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");
    if !temp_dir.is_dir() {
        fs::create_dir(&temp_dir)?;
    }
    return Ok(temp_dir);
}

pub fn get_app_songs_dir(sub_dir: Option<&str>) -> Result<PathBuf, AppError> {
    let data_dir = get_app_data_dir()?;
    let mut song_dir = data_dir.join("songs");
    if let Some(dir) = sub_dir {
        song_dir = song_dir.join(dir);
    }
    if !song_dir.is_dir() {
        fs::create_dir_all(&song_dir)?;
    }
    return Ok(song_dir);
}
