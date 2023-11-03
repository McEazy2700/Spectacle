use crate::{
    app::config::constants::VIDEO_EXTENSIONS,
    common::{errors::AppError, utils::get_cache_dir},
    server::AppState,
};
use actix_files::NamedFile;
use actix_web::{
    get,
    web::{Data, Query},
    Result,
};
use entity::entities::video_thumbnail;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

#[derive(Deserialize, Serialize)]
pub struct MediaInfo {
    path: String,
    time: Option<i32>,
    thumbnail: Option<bool>,
}

#[get("/fetch")]
pub async fn fetch_media(
    info: Query<MediaInfo>,
    data: Data<AppState>,
) -> Result<NamedFile, AppError> {
    let file_path = PathBuf::from(&info.path);

    if info.thumbnail.is_some() && info.thumbnail.unwrap() {
        let extension = match file_path.extension() {
            Some(extension) => Ok(extension),
            None => Err(AppError::new("Not a valid video file")),
        }?;

        if !VIDEO_EXTENSIONS.contains(&format!("{}", extension.to_string_lossy()).as_str()) {
            return Err(AppError::new("No a valid video file"));
        }

        let thumbnail = video_thumbnail::Entity::find_by_id(&info.path)
            .one(&data.conn)
            .await?;

        if let Some(thumbnail) = thumbnail {
            let thumbnail_path = PathBuf::from(&thumbnail.thumbnail_url);

            // Check if cache exists
            if thumbnail_path.exists() {
                return Ok(NamedFile::open(thumbnail_path)?);
            } else {
                thumbnail.delete(&data.conn).await?;
                let thumbnail = make_thumbnail(&info.path, info.time, &data.conn).await?;
                return Ok(NamedFile::open(thumbnail)?);
            }
        } else {
            let thumbnail = make_thumbnail(&info.path, info.time, &data.conn).await?;
            return Ok(NamedFile::open(thumbnail)?);
        }
    }
    return Ok(NamedFile::open(file_path)?);
}

async fn make_thumbnail(
    input_path: &String,
    time: Option<i32>,
    conn: &DbConn,
) -> anyhow::Result<PathBuf> {
    let time = time.unwrap_or(5);
    let input_buf = PathBuf::from(input_path);
    let file_name = input_buf.file_stem().unwrap();

    let cache_dir = match get_cache_dir() {
        Some(dir) => Ok(dir),
        None => Err(AppError::new("Failed to locate cache directory")),
    }?;

    let thumbnail_dir = cache_dir.join("thumbnails");

    if !thumbnail_dir.exists() {
        fs::create_dir(&thumbnail_dir)?;
    }

    let output_path = thumbnail_dir.join(format!("{}.png", file_name.to_string_lossy()));

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-ss").arg(format!("00:00:{}", time));
    cmd.arg("-i").arg(input_path);
    cmd.arg("-frames:v").arg("1");
    cmd.arg(format!("{}", &output_path.to_string_lossy()));

    let output = cmd.output();

    let new_thumbnail = video_thumbnail::ActiveModel {
        video_url: Set(format!("{}", input_path)),
        thumbnail_url: Set(format!("{}", output_path.to_string_lossy())),
    };

    match output {
        Ok(_) => {
            let new_thumbnail = new_thumbnail.insert(conn).await?;
            return Ok(PathBuf::from(new_thumbnail.thumbnail_url));
        }
        Err(err) => Err(anyhow::format_err!("ffmpeg error: {err}")),
    }
}
