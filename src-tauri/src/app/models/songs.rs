use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SongFormats {
    OpenLp,
}

impl ToString for SongFormats {
    fn to_string(&self) -> String {
        match self {
            Self::OpenLp => String::from("OpenLp"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongOptions {
    pub format: SongFormats,
    pub lang: String,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub search_title: Option<String>,
    pub search_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongUpdateOptions {
    pub id: i32,
    pub format: SongFormats,
    pub lang: String,
    pub title: Option<String>,
    pub lyrics: Option<String>,
}
