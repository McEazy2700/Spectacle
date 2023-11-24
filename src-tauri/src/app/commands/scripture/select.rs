use entity::entities::bible;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tauri::State;

use crate::{
    app::{config::db::get_bible_db_conn, models::scripture::ScriptureOptions, state::BibleDB},
    common::errors::AppError,
};

#[tauri::command]
pub async fn get_scriptures(
    opts: ScriptureOptions,
    state: State<'_, BibleDB>,
) -> Result<Vec<bible::Model>, AppError> {
    let bible_conn_arc = get_bible_db_conn(opts.version, state).await?;
    let bible_db = bible_conn_arc.lock().await;
    let scriputres = bible::Entity::find()
        .filter(bible::Column::Book.eq(opts.book))
        .filter(bible::Column::Chapter.eq(opts.chapter.to_string()));

    if let Some(limit) = opts.limit {
        let mut limit_scriptures = scriputres.cursor_by(bible::Column::StartVerse);
        let values = limit_scriptures.first(limit).all(&bible_db.to_owned()).await?;
        return Ok(values)
    };

    let values = scriputres.all(&bible_db.to_owned()).await?;
    Ok(values)
}
