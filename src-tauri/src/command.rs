use crate::*;

// no occur panic in handle fn
#[tauri::command]
pub fn on_confirm_input(app: tauri::AppHandle, text: &str) {
    log::info!("confirm input {}", text);

    match utils::hide_window_main(&app) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }

    match utils::paste_clipboard(&app, text) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_exit_input(app: tauri::AppHandle) {
    log::info!("exit input");

    match utils::hide_window_main(&app) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_change_input(app: tauri::AppHandle, text: &str) {
    // log::info!("change input {}", text);

    let state = tauri::Manager::state::<setup::AppState>(&app);

    match state.tx_input.send(text.into()) {
        Ok(_) => {},
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}
