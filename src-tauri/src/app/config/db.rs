use anyhow::anyhow;
use std::{fs, path::PathBuf};

use super::constants::APP_NAME;
use sea_orm::{Database, DbConn};

fn get_app_data_dir() -> anyhow::Result<PathBuf> {
    let config_dir = match dirs::data_dir() {
        Some(dir) => Ok(dir),
        None => Err(anyhow!("Couldn't determine config directory")),
    }?;
    let app_config_dir = config_dir.join(APP_NAME);
    if !app_config_dir.exists() {
        fs::create_dir(&app_config_dir)?
    }
    Ok(app_config_dir)
}

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
