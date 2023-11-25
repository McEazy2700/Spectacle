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
    pub text_alignment: Option<String>,
    pub vertical_alignment: Option<String>,
    pub horizontal_alignment: Option<String>,
    pub side_text_font_size: Option<i32>,
    pub side_text_font_style: Option<String>,
    pub side_text_font_color: Option<String>,
    pub side_text_font_weight: Option<i32>,
    pub side_text_text_alignment: Option<String>,
    pub side_text_vertical_alignment: Option<String>,
    pub side_text_horizontal_alignment: Option<String>,
}
