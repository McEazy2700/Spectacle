use crate::{
    app::{models::template::TemplateOptions, state::DB},
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
    opts: TemplateOptions,
) -> Result<template::Model, AppError> {
    if let Some(template_id) = opts.id {
        return Ok(update_template(&state.conn, template_id, opts).await?);
    } else {
        return Ok(new_template(&state.conn, opts).await?);
    }
}

async fn new_template(conn: &DbConn, opts: TemplateOptions) -> Result<template::Model, AppError> {
    if !opts.name.is_some() {
        return Err(AppError::new("Template name is required"));
    };
    if !opts.font_size.is_some() {
        return Err(AppError::new("Template font_size is required"));
    }
    if !opts.font_style.is_some() {
        return Err(AppError::new("Template font_style is required"));
    }
    if !opts.font_weight.is_some() {
        return Err(AppError::new("Template font_weight is required"));
    }
    if !opts.text_alignment.is_some() {
        return Err(AppError::new("Template text_alignment is required"));
    }
    if !opts.vertical_alignment.is_some() {
        return Err(AppError::new("Template vertical_alignment is required"));
    }
    if !opts.horizontal_alignment.is_some() {
        return Err(AppError::new("Template horizontal_alignment is required"));
    }
    if !opts.side_text_font_size.is_some() {
        return Err(AppError::new("Template side_text_font_size is required"));
    }
    if !opts.side_text_font_style.is_some() {
        return Err(AppError::new("Template side_text_font_style is required"));
    }
    if !opts.side_text_font_weight.is_some() {
        return Err(AppError::new("Template side_text_font_weight is required"));
    }
    if !opts.side_text_text_alignment.is_some() {
        return Err(AppError::new(
            "Template side_text_text_alignment is required",
        ));
    }
    if !opts.side_text_vertical_alignment.is_some() {
        return Err(AppError::new(
            "Template side_text_vertical_alignment is required",
        ));
    }
    if !opts.side_text_horizontal_alignment.is_some() {
        return Err(AppError::new(
            "Template side_text_horizontal_alignment is required",
        ));
    }
    let new_template = template::ActiveModel {
        name: Set(opts.name.unwrap()),
        font_size: Set(opts.font_size.unwrap()),
        font_style: Set(opts.font_style.unwrap()),
        font_color: Set(opts.font_color),
        background: Set(opts.background),
        font_weight: Set(opts.font_weight.unwrap()),
        text_alignment: Set(opts.text_alignment.unwrap()),
        vertical_alignment: Set(opts.vertical_alignment.unwrap()),
        horizontal_alignment: Set(opts.horizontal_alignment.unwrap()),
        side_text_font_size: Set(opts.side_text_font_size.unwrap()),
        side_text_font_style: Set(opts.side_text_font_style.unwrap()),
        side_text_font_color: Set(opts.side_text_font_color),
        side_text_font_weight: Set(opts.side_text_font_weight.unwrap()),
        side_text_text_alignment: Set(opts.side_text_text_alignment.unwrap()),
        side_text_vertical_alignment: Set(opts.side_text_vertical_alignment.unwrap()),
        side_text_horizontal_alignment: Set(opts.side_text_horizontal_alignment.unwrap()),
        ..Default::default()
    };
    return Ok(new_template.insert(conn).await?)
}

async fn update_template(
    conn: &DbConn,
    template_id: i32,
    opts: TemplateOptions,
) -> Result<template::Model, AppError> {
    if let Some(temp) = template::Entity::find_by_id(template_id).one(conn).await? {
        let mut temp: template::ActiveModel = temp.into();
        if opts.name.is_some() {
            temp.name = Set(opts.name.unwrap());
        }
        if opts.font_size.is_some() {
            temp.font_size = Set(opts.font_size.unwrap());
        }
        if opts.font_style.is_some() {
            temp.font_style = Set(opts.font_style.unwrap());
        }
        if opts.font_color.is_some() {
            temp.font_color = Set(opts.font_color);
        }
        if opts.background.is_some() {
            temp.background = Set(opts.background);
        }
        if opts.font_weight.is_some() {
            temp.font_weight = Set(opts.font_weight.unwrap());
        }
        if opts.text_alignment.is_some() {
            temp.text_alignment = Set(opts.text_alignment.unwrap());
        }
        if opts.vertical_alignment.is_some() {
            temp.vertical_alignment = Set(opts.vertical_alignment.unwrap());
        }
        if opts.horizontal_alignment.is_some() {
            temp.horizontal_alignment = Set(opts.horizontal_alignment.unwrap());
        }
        if opts.side_text_font_size.is_some() {
            temp.side_text_font_size = Set(opts.side_text_font_size.unwrap());
        }
        if opts.side_text_font_style.is_some() {
            temp.side_text_font_style = Set(opts.side_text_font_style.unwrap());
        }
        if opts.side_text_font_color.is_some() {
            temp.side_text_font_color = Set(opts.side_text_font_color);
        }
        if opts.side_text_font_weight.is_some() {
            temp.side_text_font_weight = Set(opts.side_text_font_weight.unwrap());
        }
        if opts.side_text_text_alignment.is_some() {
            temp.side_text_text_alignment = Set(opts.side_text_text_alignment.unwrap());
        }
        if opts.side_text_vertical_alignment.is_some() {
            temp.side_text_vertical_alignment = Set(opts.side_text_vertical_alignment.unwrap());
        }
        if opts.side_text_horizontal_alignment.is_some() {
            temp.side_text_horizontal_alignment = Set(opts.side_text_horizontal_alignment.unwrap());
        }
        return Ok(temp.update(conn).await?);
    } else {
        return Err(AppError::new("Template was not found"));
    }
}
