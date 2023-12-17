use crate::{
    app::{config::db::get_song_db_conn, models::songs::SongOptions, state::SongDB},
    common::errors::AppError,
};
use entity::entities::songs;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use tauri::State;

#[tauri::command]
pub async fn get_songs(
    opts: SongOptions,
    state: State<'_, SongDB>,
) -> Result<Vec<songs::Model>, AppError> {
    let songs_db_arc = get_song_db_conn(&opts.format, &opts.lang, state).await?;
    let songs_db = songs_db_arc.lock().await;
    let mut query = songs::Entity::find();
    if let Some(search) = opts.search_text {
        query = query.filter(songs::Column::SearchLyrics.contains(search));
    }
    if let Some(search) = opts.search_title {
        query = query.filter(songs::Column::SearchTitle.contains(search));
    }
    let mut song_cursor = query.cursor_by(songs::Column::Id).offset(opts.offset);

    if let Some(limit) = opts.limit {
        let values = song_cursor.first(limit).all(&songs_db.to_owned()).await?;
        return Ok(values);
    } else {
        let values = song_cursor.all(&songs_db.to_owned()).await?;
        return Ok(values);
    }
}
