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
use app::commands::scripture::queries::get_scriptures;
use app::commands::songs::download::{download_openlp_song, get_downloaded_song_dbs};
use app::commands::songs::mutations::update_song;
use app::commands::songs::queries::get_songs;
use app::commands::template::{
    get_default_template, get_template, get_templates, save_template, set_default_template,
};
use app::commands::view::{get_live_view, set_live_view};
use app::state::{BibleDB, Live, SongDB};
use app::{config::db, state::DB};
use migration::{Migrator, MigratorTrait};
use std::thread;
use tauri::async_runtime::block_on;

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
        .manage(BibleDB::new())
        .manage(SongDB::new())
        .manage(Live::new())
        .invoke_handler(tauri::generate_handler![
            get_songs,
            update_song,
            cleanup_temp,
            get_template,
            save_template,
            get_templates,
            set_live_view,
            get_live_view,
            scan_media_dir,
            download_bible,
            get_scriptures,
            parse_bible_sql,
            create_bible_db,
            extract_bible_zip,
            download_openlp_song,
            get_default_template,
            set_default_template,
            get_downloaded_song_dbs,
            get_downloaded_bible_versions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
