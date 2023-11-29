use crate::{
    app::{models::slide::View, state::Live},
    common::errors::AppError,
};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn set_live_view(
    view: View,
    state: State<'_, Live>,
    app: AppHandle,
) -> Result<Option<View>, AppError> {
    let mut live_view = state.view.lock().unwrap();
    *live_view = Some(view);
    let owned_view = live_view.to_owned();
    app.emit_all("state-update", &owned_view)?;
    return Ok(owned_view);
}

#[tauri::command]
pub fn get_live_view(state: State<'_, Live>) -> Option<View> {
    let live_view = state.view.lock().unwrap();
    return live_view.to_owned();
}
