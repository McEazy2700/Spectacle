use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptureOptions {
    pub version: String,
    pub book: String,
    pub chapter: i32,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BibleVerse {
    pub version: String,
    pub book: String,
    pub chapter: String,
    pub verse: u32,
    pub content: String,
}
