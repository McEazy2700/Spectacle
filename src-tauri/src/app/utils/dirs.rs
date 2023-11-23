use anyhow::anyhow;
use std::{fs, path::PathBuf};

use crate::app::config::constants::APP_NAME;

pub fn get_app_data_dir() -> anyhow::Result<PathBuf> {
    let config_dir = match dirs::data_dir() {
        Some(dir) => Ok(dir),
        None => Err(anyhow!("Couldn't determine config directory")),
    }?;
    let app_config_dir = config_dir.join(APP_NAME);
    if !app_config_dir.exists() {
        fs::create_dir(&app_config_dir)?
    }
    Ok(app_config_dir)
}

pub fn get_bible_dir() -> anyhow::Result<PathBuf> {
    let data_dir = get_app_data_dir()?;
    let bible_dir = data_dir.join("bible");
    if !bible_dir.is_dir() {
        fs::create_dir(&bible_dir)?;
    }
    return Ok(bible_dir);
}
