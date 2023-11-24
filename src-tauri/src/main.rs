// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod common;
pub mod server;

use app::commands::media::scan_media_dir;
use app::commands::scripture::download::{
    cleanup_temp, create_bible_db, download_bible, extract_bible_zip, parse_bible_sql,
};
use app::commands::scripture::get_downloaded_bible_versions;
use app::commands::scripture::select::get_scriptures;
use app::commands::template::{get_templates, save_template};
use app::commands::view::{get_live_view, set_live_view};
use app::state::{BibleDB, Live};
use app::{config::db, state::DB};
use migration::{Migrator, MigratorTrait};
use std::{collections::HashMap, thread};
use tauri::async_runtime::block_on;
use tokio::sync::Mutex;

fn main() {
    let db_conn = match block_on(db::get_db_connection()) {
        Ok(conn) => conn,
        Err(err) => panic!("Database Connection Error: {err}"),
    };
    match block_on(Migrator::up(&db_conn, None)) {
        Ok(_) => {}
        Err(err) => panic!("Database Migration Error: {err}"),
    };
    let server_db_con = db_conn.clone();
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || server::run(*boxed_handle, server_db_con).unwrap());
            Ok(())
        })
        .manage(DB { conn: db_conn })
        .manage(BibleDB {
            conns: Mutex::new(HashMap::new()),
        })
        .manage(Live {
            view: std::sync::Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            scan_media_dir,
            save_template,
            get_templates,
            set_live_view,
            get_live_view,
            get_downloaded_bible_versions,
            download_bible,
            extract_bible_zip,
            parse_bible_sql,
            create_bible_db,
            cleanup_temp,
            get_scriptures
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
