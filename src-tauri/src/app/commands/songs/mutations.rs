use crate::{
    app::{config::db::get_song_db_conn, models::songs::SongUpdateOptions, state::SongDB},
    common::errors::AppError,
};
use entity::entities::songs;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use tauri::State;

#[tauri::command]
pub async fn update_song(
    opts: SongUpdateOptions,
    state: State<'_, SongDB>,
) -> Result<songs::Model, AppError> {
    let song_db_arc = get_song_db_conn(&opts.format, &opts.lang, state).await?;
    let song_db = song_db_arc.lock().await;

    let song = songs::Entity::find_by_id(opts.id)
        .one(&song_db.to_owned())
        .await?;

    match song {
        Some(song) => {
            let mut song: songs::ActiveModel = song.into();
            if let Some(lyrics) = opts.lyrics {
                song.lyrics = Set(lyrics);
            }
            if let Some(title) = opts.title {
                song.title = Set(title);
            }

            let song = song.update(&song_db.to_owned()).await?;
            return Ok(song);
        }
        None => Err(AppError::new(
            format!(
                "Song with id {}, lang {} and format {} was not found",
                &opts.id,
                &opts.lang,
                &opts.format.to_string()
            )
            .as_str(),
        )),
    }
}
