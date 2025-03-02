mod setup;

// enable mobile entry point when mobile target
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            setup::greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
