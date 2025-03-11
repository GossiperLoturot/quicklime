mod command;
mod setup;

// enable mobile entry point when mobile target
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    tauri::Builder::default()
        .plugin(setup::plugin_clipboard())
        .plugin(setup::plugin_global_shortcut())
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            command::on_confirm_input,
            command::on_exit_input,
        ])
        .on_window_event(setup::on_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
