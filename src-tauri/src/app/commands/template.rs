use crate::app::models::template::{BackgroundType, FontWeight};
use crate::{
    app::{models::template::TemplateOption, state::DB},
    common::errors::AppError,
};
use entity::entities::template;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait};
use tauri::State;

#[tauri::command]
pub async fn get_templates(state: State<'_, DB>) -> Result<Vec<template::Model>, AppError> {
    let templates = template::Entity::find().all(&state.conn).await?;
    return Ok(templates);
}

#[tauri::command]
pub async fn save_template(
    state: State<'_, DB>,
    opts: TemplateOption,
) -> Result<template::Model, AppError> {
    if let Some(template_id) = opts.id {
        return Ok(update_template(&state.conn, template_id, opts).await?);
    } else {
        return Ok(new_template(&state.conn, opts).await?);
    }
}

async fn new_template(conn: &DbConn, opts: TemplateOption) -> Result<template::Model, AppError> {
    if !opts.name.is_some() {
        return Err(AppError::new("Template name is required"));
    };
    let new_template = template::ActiveModel {
        name: Set(opts.name.unwrap()),
        font_size: Set(opts.font_size),
        font_weight: Set(Some(
            opts.font_weight.unwrap_or(FontWeight::Normal).to_string(),
        )),
        background_url: Set(opts.background_url),
        background_type: Set(Some(
            opts.background_type
                .unwrap_or(BackgroundType::Image)
                .to_string(),
        )),
        ..Default::default()
    };
    return Ok(new_template.insert(conn).await?);
}

async fn update_template(
    conn: &DbConn,
    template_id: i32,
    opts: TemplateOption,
) -> Result<template::Model, AppError> {
    if let Some(temp) = template::Entity::find_by_id(template_id).one(conn).await? {
        let mut temp: template::ActiveModel = temp.into();
        if opts.name.is_some() {
            temp.name = Set(opts.name.unwrap())
        }
        if opts.font_size.is_some() {
            temp.font_size = Set(opts.font_size)
        }
        if opts.font_weight.is_some() {
            temp.font_weight = Set(Some(opts.font_weight.unwrap().to_string()))
        }
        if opts.background_url.is_some() {
            temp.background_url = Set(opts.background_url)
        }
        if opts.background_type.is_some() {
            temp.background_type = Set(Some(opts.background_type.unwrap().to_string()))
        }
        return Ok(temp.update(conn).await?);
    } else {
        return Err(AppError::new("Template was not found"));
    }
}
