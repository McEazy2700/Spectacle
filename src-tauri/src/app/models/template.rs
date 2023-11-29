use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateOptions {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub font_size: Option<i32>,
    pub font_style: Option<String>,
    pub font_color: Option<String>,
    pub background: Option<String>,
    pub font_weight: Option<i32>,
    pub text_shadow: Option<bool>,
    pub text_shadow_blur: Option<i32>,
    pub text_shadow_color: Option<String>,
    pub text_shadow_vertical: Option<i32>,
    pub text_shadow_horizontal: Option<i32>,
    pub text_alignment: Option<String>,
    pub vertical_alignment: Option<String>,
    pub horizontal_alignment: Option<String>,
    pub side_text_font_size: Option<i32>,
    pub side_text_font_style: Option<String>,
    pub side_text_font_color: Option<String>,
    pub side_text_font_weight: Option<i32>,
    pub side_text_shadow: Option<bool>,
    pub side_text_shadow_blur: Option<i32>,
    pub side_text_shadow_color: Option<String>,
    pub side_text_shadow_vertical: Option<i32>,
    pub side_text_shadow_horizontal: Option<i32>,
    pub side_text_text_alignment: Option<String>,
    pub side_text_vertical_alignment: Option<String>,
    pub side_text_horizontal_alignment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TemplateView {
    Song,
    Text,
    Media,
    Sermon,
    Scripture,
}

impl ToString for TemplateView {
    fn to_string(&self) -> String {
        match self {
            Self::Song => String::from("Song"),
            Self::Text => String::from("Text"),
            Self::Media => String::from("Media"),
            Self::Sermon => String::from("Sermon"),
            Self::Scripture => String::from("Scripture"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultTemplateOpts {
    pub view: TemplateView,
    pub template_id: i32,
}
