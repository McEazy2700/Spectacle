use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaSlide {
    id: i32,
    src: String,
    template_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptureSlide {
    id: i32,
    r#type: String,
    text: String,
    passage: String,
    template_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    show: bool,
    r#type: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SermonSlide {
    id: i32,
    r#type: String,
    text: String,
    alert: Option<Alert>,
    template_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongSlide {
    id: i32,
    r#type: String,
    text: String,
    verse: i32,
    template_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextSlide {
    id: i32,
    r#type: String,
    text: String,
    template_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SlideType {
    Media(MediaSlide),
    Sermon(SermonSlide),
    Scripture(ScriptureSlide),
    Song(SongSlide),
    Text(TextSlide),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct View {
    id: i32,
    name: String,
    next: Option<i32>,
    prev: Option<i32>,
    slides: Vec<SlideType>,
}

pub type ScheduleType = Vec<View>;
