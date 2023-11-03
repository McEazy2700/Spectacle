use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DbConn;
use std::sync::Mutex;
use tauri::AppHandle;

use self::handlers::media;

pub mod handlers;

pub struct AppState {
    pub app: Mutex<AppHandle>,
    pub conn: DbConn,
}

#[actix_web::main]
pub async fn run(app: AppHandle, conn: DbConn) -> std::io::Result<()> {
    let tauri_app = web::Data::new(AppState {
        app: Mutex::new(app),
        conn,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/media").service(media::fetch_media))
    })
    .bind(("127.0.0.1", 2700))?
    .run()
    .await
}
