use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptureOptions {
    pub version: String,
    pub book: String,
    pub chapter: i32,
    pub start_verse: i32,
    pub limit: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BibleVerse {
    pub version: String,
    pub book: String,
    pub chapter: String,
    pub verse: u32,
    pub content: String
}
