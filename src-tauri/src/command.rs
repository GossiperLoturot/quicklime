use crate::*;

#[tauri::command]
pub fn on_confirm_input(app: tauri::AppHandle, text: &str) {
    log::info!("confirm input {}", text);

    utils::hide_window_main(&app).unwrap();

    utils::paste_clipboard(&app, text).unwrap();
}

#[tauri::command]
pub fn on_exit_input(app: tauri::AppHandle) {
    log::info!("exit input");

    utils::hide_window_main(&app).unwrap();
}

#[tauri::command]
pub fn on_change_input(text: &str) {
    log::info!("change input {}", text);
}
