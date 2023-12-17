use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    app::{
        models::songs::SongFormats,
        state::{BibleDB, SongDB},
        utils::dirs::{get_app_data_dir, get_app_songs_dir, get_bible_dir},
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
            let config_dir = get_bible_dir()?;
            let database_file = config_dir.join(&bible_version);
            let db_conn =
                connect_and_store(bible_version, database_file, "db".to_string(), &state.conns)
                    .await?;
            Ok(db_conn)
        }
    }
}

pub async fn get_song_db_conn(
    format: &SongFormats,
    lang: &String,
    state: State<'_, SongDB>,
) -> Result<Arc<Mutex<DbConn>>, AppError> {
    let fmt_low = format.to_string().to_lowercase();
    let key = format!("{}/{}", &fmt_low, &lang.to_lowercase());
    match get_connection(&key, &state.conns).await {
        Some(conn) => Ok(conn),
        None => {
            let song_dir = get_app_songs_dir(Some(fmt_low.as_str()))?;
            let db_file = song_dir.join(&lang);
            let conn = connect_and_store(key, db_file, "sqlite".to_string(), &state.conns).await?;
            Ok(conn)
        }
    }
}

async fn get_connection(
    key: &String,
    conns: &Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
) -> Option<Arc<Mutex<DbConn>>> {
    let conns = conns.lock().await;
    let conn = conns.get(key);
    match conn {
        Some(conn) => Some(Arc::clone(conn)),
        None => None,
    }
}

/// Creates and stores a database connection
/// # Parameters
/// `key`: The key with which to store the connection
/// `path`: The path to the database file `without the extension!`
/// `extension`: The extension of the database file e.g db or sqlite
/// `conns`: A mutable HashMap for stroring the database connection
///
/// # Example
/// ```rust
///  let song_dir = get_app_songs_dir(Some(fmt_low.as_str()))?;
///  let db_file = song_dir.join(&lang);
///  connect_and_store(&key, db_file, "sqlite".to_string(), &state.conns)
/// ```
async fn connect_and_store(
    key: String,
    path: PathBuf,
    extension: String,
    conns: &Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
) -> Result<Arc<Mutex<DbConn>>, AppError> {
    let db_conn = connect_file_db(path, extension).await?;
    let conn = store_db_connection(conns, key, db_conn).await?;
    return Ok(conn);
}

async fn connect_file_db(path: PathBuf, db_extension: String) -> Result<DbConn, AppError> {
    let database_connection = Database::connect(&format!(
        "sqlite://{}.{}?mode=rwc",
        path.to_string_lossy(),
        db_extension
    ))
    .await?;
    return Ok(database_connection);
}

async fn store_db_connection(
    conns: &Mutex<HashMap<String, Arc<Mutex<DbConn>>>>,
    key: String,
    conn: DbConn,
) -> Result<Arc<Mutex<DbConn>>, AppError> {
    let conn = Arc::new(Mutex::new(conn));
    let mut conns_mutex = conns.lock().await;
    conns_mutex.insert(key, conn.clone());
    Ok(conn)
}
