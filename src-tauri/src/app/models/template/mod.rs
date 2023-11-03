use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BgType {
    Video,
    Image,
}

impl ToString for BgType {
    fn to_string(&self) -> String {
        match self {
            BgType::Video => String::from("Video"),
            BgType::Image => String::from("Image"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
            FontWeight::Bold => String::from("Bold"),
            FontWeight::Light => String::from("Light"),
            FontWeight::Normal => String::from("Normal"),
            FontWeight::SemiBold => String::from("SemiBold"),
            FontWeight::ExtraBold => String::from("ExtraBold"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateOption {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub font_size: Option<i32>,
    pub font_weight: Option<FontWeight>,
    pub background_url: Option<String>,
    pub background_type: Option<BgType>,
}
