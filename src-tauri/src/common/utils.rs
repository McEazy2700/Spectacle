use std::{fs, path::PathBuf};

use crate::app::config::constants::APP_NAME;

pub fn get_cache_dir() -> Option<PathBuf> {
    let cache_dir = dirs::cache_dir()?;
    let app_cache_dir = cache_dir.join(APP_NAME);
    if app_cache_dir.exists() {
        return Some(app_cache_dir);
    } else {
        fs::create_dir(&app_cache_dir).unwrap();
        return Some(app_cache_dir);
    }
}
