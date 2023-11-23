use std::{collections::HashMap, sync::Arc};
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    app::{
        state::BibleDB,
        utils::dirs::{get_app_data_dir, get_bible_dir},
    },
    common::errors::AppError,
};

use sea_orm::{Database, DbConn};

pub async fn get_db_connection() -> anyhow::Result<DbConn> {
    let config_dir = get_app_data_dir()?;
    let database_file = config_dir.join("db.sqlite3");
    let conn = Database::connect(&format!(
        "sqlite://{}?mode=rwc",
        database_file.to_string_lossy()
    ))
    .await?;
    Ok(conn)
}

pub async fn get_bible_db_conn(
    bible_version: String,
    state: State<'_, BibleDB>,
) -> Result<Arc<Mutex<DbConn>>, AppError> {
    match get_connection(&bible_version, &state.conns).await {
        Some(conn) => Ok(conn),
        None => {
            let db_conn = connect_and_store(&bible_version, &state.conns).await?;
            Ok(db_conn)
        }
    }
}

async fn get_connection(
    bible_db_name: &String,
    conns: &Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
) -> Option<Arc<Mutex<DbConn>>> {
    let conns = conns.lock().await;
    let conn = conns.get(bible_db_name);
    match conn {
        Some(conn) => Some(Arc::clone(conn)),
        None => None,
    }
}

async fn connect_and_store(
    bible_db_name: &String,
    conns: &Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
) -> Result<Arc<Mutex<DbConn>>, AppError> {
    let config_dir = get_bible_dir()?;
    let database_file = config_dir.join(&bible_db_name);
    let database_connection = Database::connect(&format!(
        "sqlite://{}.db?mode=rwc",
        database_file.to_string_lossy()
    ))
    .await?;
    let conn = Arc::new(Mutex::new(database_connection));
    let mut conns_mutex = conns.lock().await;
    conns_mutex.insert(bible_db_name.clone(), conn.clone());
    Ok(conn)
}
