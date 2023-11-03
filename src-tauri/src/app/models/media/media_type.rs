use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

impl MediaType {
    pub fn get_dir(&self) -> Option<PathBuf> {
        match self {
            MediaType::Image => dirs::picture_dir(),
            MediaType::Audio => dirs::audio_dir(),
            MediaType::Video => dirs::video_dir(),
        }
    }
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        match self {
            MediaType::Image => String::from("image"),
            MediaType::Video => String::from("video"),
            MediaType::Audio => String::from("audio"),
        }
    }
}
