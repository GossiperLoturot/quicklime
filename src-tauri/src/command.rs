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
pub fn on_change_input(app: tauri::AppHandle, input: &str, mode: usize) {
    let state = tauri::Manager::state::<setup::AppState>(&app);

    state.mode.store(mode, std::sync::atomic::Ordering::Relaxed);

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
pub async fn on_change_config(app: tauri::AppHandle, config: setup::Config) -> Result<(), ()> {
    log::info!("change config {:?}", config);

    let state = tauri::Manager::state::<setup::AppState>(&app);
    let mut current_config = state.config.lock().await;
    *current_config = config.clone();

    let path = match tauri::Manager::path(&app).app_config_dir() {
        Ok(path) => path,
        Err(e) => {
            log::error!("error occured {}", e);
            return Err(());
        }
    };
    match std::fs::create_dir_all(&path) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return Err(());
        }
    }
    let filepath = &path.join("config.json");
    log::info!("save config {:?}", filepath);
    let writer = match std::fs::File::create(&filepath) {
        Ok(writer) => writer,
        Err(e) => {
            log::error!("error occured {}", e);
            return Err(());
        }
    };
    match serde_json::to_writer(writer, &config) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return Err(());
        }
    };

    Ok(())
}

// no occur panic in handle fn
#[tauri::command]
pub async fn get_config(app: tauri::AppHandle) -> Result<setup::Config, ()> {
    log::info!("get config");

    let state = tauri::Manager::state::<setup::AppState>(&app);
    let current_config = state.config.lock().await;

    Ok(current_config.clone())
}
