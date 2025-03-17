use crate::*;

// no occur panic in handle fn
#[tauri::command]
pub fn on_confirm_input(app: tauri::AppHandle, input: &str) {
    log::info!("confirm input {}", input);

    match utils::hide_window_main(&app) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }

    match utils::paste_clipboard(&app, input) {
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
pub fn on_change_input(app: tauri::AppHandle, input: &str) {
    // log::info!("change input {}", text);

    let state = tauri::Manager::state::<setup::AppState>(&app);

    match state.tx_input.send(input.into()) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_change_mode(app: tauri::AppHandle, mode: usize) {
    log::info!("change mode {}", mode);

    match tauri::Emitter::emit(&app, "update_mode", mode) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }

    let state = tauri::Manager::state::<setup::AppState>(&app);
    state.mode.store(mode, std::sync::atomic::Ordering::Relaxed);
}
