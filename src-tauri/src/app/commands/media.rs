use crate::{app::models::media::media_type::MediaType, common::errors::AppError};
use std::{fs, path::PathBuf};

#[tauri::command]
pub fn scan_media_dir(
    media_type: MediaType,
    sub_dir: Option<String>, // A full path to a sub_directory e.g. /home/Vice/Pictures/Wallpaper/
) -> Result<Vec<PathBuf>, AppError> {
    let media_dir = match sub_dir {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => match media_type.get_dir() {
            Some(dir) => Ok(dir),
            None => Err(AppError::new("Directory not found")),
        },
    }?;

    let dir = match fs::read_dir(media_dir) {
        Ok(dir) => Ok(dir),
        Err(err) => Err(AppError::new(err.to_string().as_str())),
    }?;

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    directories.push(entry.path().join(""))
                } else {
                    files.push(entry.path())
                }
            }
        }
    }

    directories.sort();
    files.sort();
    directories.append(&mut files);
    return Ok(directories);
}
