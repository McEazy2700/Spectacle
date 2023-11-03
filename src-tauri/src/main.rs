// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod common;
pub mod server;

use app::commands::{
    media::scan_media_dir,
    template::{get_templates, save_template},
};
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
        .invoke_handler(tauri::generate_handler![
            scan_media_dir,
            save_template,
            get_templates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
