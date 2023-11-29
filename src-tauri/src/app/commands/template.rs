use crate::{
    app::{
        models::template::{DefaultTemplateOpts, TemplateOptions, TemplateView},
        state::DB,
    },
    common::errors::AppError,
};
use entity::entities::{default_template, template};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, QueryFilter};
use tauri::State;

#[tauri::command]
pub async fn set_default_template(
    opts: DefaultTemplateOpts,
    state: State<'_, DB>,
) -> Result<default_template::Model, AppError> {
    let new_def_temp = default_template::ActiveModel {
        view: Set(opts.view.to_string()),
        template_id: Set(opts.template_id),
        ..Default::default()
    };
    let def_template = new_def_temp.insert(&state.conn).await?;
    return Ok(def_template);
}

#[tauri::command]
pub async fn get_default_template(
    view: TemplateView,
    state: State<'_, DB>,
) -> Result<default_template::Model, AppError> {
    let def_temp = default_template::Entity::find()
        .filter(default_template::Column::View.eq(view.to_string()));
    let def_temp = def_temp.one(&state.conn).await?;
    match def_temp {
        Some(temp) => Ok(temp),
        None => Err(AppError::new(
            format!("Default template for {} was not found", view.to_string()).as_str(),
        )),
    }
}

#[tauri::command]
pub async fn get_templates(state: State<'_, DB>) -> Result<Vec<template::Model>, AppError> {
    let templates = template::Entity::find().all(&state.conn).await?;
    return Ok(templates);
}

#[tauri::command]
pub async fn get_template(id: i32, state: State<'_, DB>) -> Result<template::Model, AppError> {
    let template = template::Entity::find_by_id(id).one(&state.conn).await?;
    match template {
        Some(template) => Ok(template),
        None => Err(AppError::new(
            format!("Template with id {} was not found", id).as_str(),
        )),
    }
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
    if !opts.text_shadow.is_some() {
        return Err(AppError::new("Template text_shadow is required"));
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
    if !opts.side_text_shadow.is_some() {
        return Err(AppError::new("Template side_text_shadow is required"));
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
        text_shadow: Set(opts.text_shadow.unwrap()),
        text_shadow_blur: Set(opts.text_shadow_blur),
        text_shadow_color: Set(opts.text_shadow_color),
        text_shadow_vertical: Set(opts.text_shadow_vertical),
        text_shadow_horizontal: Set(opts.text_shadow_horizontal),
        text_alignment: Set(opts.text_alignment.unwrap()),
        vertical_alignment: Set(opts.vertical_alignment.unwrap()),
        horizontal_alignment: Set(opts.horizontal_alignment.unwrap()),
        side_text_font_size: Set(opts.side_text_font_size.unwrap()),
        side_text_font_style: Set(opts.side_text_font_style.unwrap()),
        side_text_font_color: Set(opts.side_text_font_color),
        side_text_font_weight: Set(opts.side_text_font_weight.unwrap()),
        side_text_shadow: Set(opts.side_text_shadow.unwrap()),
        side_text_shadow_blur: Set(opts.side_text_shadow_blur),
        side_text_shadow_color: Set(opts.side_text_shadow_color),
        side_text_shadow_vertical: Set(opts.side_text_shadow_vertical),
        side_text_shadow_horizontal: Set(opts.side_text_shadow_horizontal),
        side_text_text_alignment: Set(opts.side_text_text_alignment.unwrap()),
        side_text_vertical_alignment: Set(opts.side_text_vertical_alignment.unwrap()),
        side_text_horizontal_alignment: Set(opts.side_text_horizontal_alignment.unwrap()),
        ..Default::default()
    };
    return Ok(new_template.insert(conn).await?);
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
        if opts.text_shadow.is_some() {
            temp.text_shadow = Set(opts.text_shadow.unwrap());
        }
        if opts.text_shadow_blur.is_some() {
            temp.text_shadow_blur = Set(opts.text_shadow_blur);
        }
        if opts.text_shadow_color.is_some() {
            temp.text_shadow_color = Set(opts.text_shadow_color);
        }
        if opts.text_shadow_vertical.is_some() {
            temp.text_shadow_vertical = Set(opts.text_shadow_vertical);
        }
        if opts.text_shadow_horizontal.is_some() {
            temp.text_shadow_horizontal = Set(opts.text_shadow_horizontal);
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
        if opts.side_text_shadow.is_some() {
            temp.side_text_shadow = Set(opts.side_text_shadow.unwrap());
        }
        if opts.side_text_shadow_blur.is_some() {
            temp.side_text_shadow_blur = Set(opts.side_text_shadow_blur);
        }
        if opts.side_text_shadow_color.is_some() {
            temp.side_text_shadow_color = Set(opts.side_text_shadow_color);
        }
        if opts.side_text_shadow_vertical.is_some() {
            temp.side_text_shadow_vertical = Set(opts.side_text_shadow_vertical);
        }
        if opts.side_text_shadow_horizontal.is_some() {
            temp.side_text_shadow_horizontal = Set(opts.side_text_shadow_horizontal);
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
