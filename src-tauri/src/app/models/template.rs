use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FontWeight {
    Light,
    Normal,
    SemiBold,
    Bold,
    ExtraBold,
}

impl ToString for FontWeight {
    fn to_string(&self) -> String {
        match self {
            FontWeight::Bold => String::from("bold"),
            FontWeight::Light => String::from("light"),
            FontWeight::Normal => String::from("normal"),
            FontWeight::SemiBold => String::from("semibold"),
            FontWeight::ExtraBold => String::from("extrabold"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

impl ToString for Alignment {
    fn to_string(&self) -> String {
        match self {
            Alignment::End => String::from("end"),
            Alignment::Start => String::from("start"),
            Alignment::Center => String::from("center"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackgroundType {
    Video,
    Image,
}

impl ToString for BackgroundType {
    fn to_string(&self) -> String {
        match self {
            BackgroundType::Video => String::from("video"),
            BackgroundType::Image => String::from("image"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Background {
    pub r#type: BackgroundType,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectFit {
    Fill,
    Cover,
    Contain,
}

impl ToString for ObjectFit {
    fn to_string(&self) -> String {
        match self {
            ObjectFit::Fill => String::from("fill"),
            ObjectFit::Cover => String::from("cover"),
            ObjectFit::Contain => String::from("contain")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaTemplateModel {
    object_fit: ObjectFit
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateModel {
    pub font: Option<String>,
    pub font_size: i32,
    pub font_weight: FontWeight,
    pub vertical_alignment: Alignment,
    pub horizontal_alignment: Alignment,
    pub background: Background,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateOption {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub font_size: Option<i32>,
    pub font_weight: Option<FontWeight>,
    pub vertical_alignment: Option<Alignment>,
    pub horizontal_alignment: Option<Alignment>,
    pub background_url: Option<String>,
    pub background_type: Option<BackgroundType>,
}
