use tauri::{State, AppHandle, Manager};
use crate::{app::{state::Live, models::slide::View}, common::errors::AppError};

#[tauri::command]
pub fn set_live_view(view: View, state: State<Live>, app: AppHandle) -> Result<Option<View>, AppError> {
    let mut live_view = state.view.lock().unwrap();
    *live_view = Some(view);
    let owned_view = live_view.to_owned();
    app.emit_all("state-update", &owned_view)?;
    return Ok(owned_view)
}

#[tauri::command]
pub fn get_live_view(view: View, state: State<Live>) -> Option<View> {
    let mut live_view = state.view.lock().unwrap();
    *live_view = Some(view);
    return live_view.to_owned()
}
