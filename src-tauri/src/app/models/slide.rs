use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaSlide {
    id: String,
    r#type: String,
    src: String,
    template_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptureSlide {
    id: String,
    r#type: String,
    text: String,
    passage: String,
    template_id: i32,
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
    id: String,
    r#type: String,
    text: String,
    template_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongSlide {
    id: String,
    r#type: String,
    text: String,
    verse: i32,
    template_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextSlide {
    id: String,
    r#type: String,
    text: String,
    template_id: i32,
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
    id: String,
    name: String,
    r#type: String,
    slide: SlideType,
}

pub type ScheduleType = Vec<View>;
