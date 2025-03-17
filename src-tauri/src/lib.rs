mod command;
mod setup;
mod utils;

// enable mobile entry point when mobile target
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;

    tauri::Builder::default()
        .plugin(setup::setup_plugin_clipboard()?)
        .plugin(setup::setup_plugin_global_shortcut()?)
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            command::on_confirm_input,
            command::on_exit_input,
            command::on_change_input,
            command::on_change_mode,
        ])
        .on_window_event(setup::setup_window_event()?)
        .run(tauri::generate_context!())?;
    Ok(())
}
