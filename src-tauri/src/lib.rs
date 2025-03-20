mod app;
mod llm;

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
        .plugin(app::setup_plugin_clipboard()?)
        .plugin(app::setup_plugin_global_shortcut()?)
        .setup(app::setup)
        .invoke_handler(tauri::generate_handler![
            app::on_confirm_input,
            app::on_exit_input,
            app::on_change_input,
            app::on_change_config,
            app::on_get_config,
        ])
        .on_window_event(app::setup_window_event_handle()?)
        .run(tauri::generate_context!())?;
    Ok(())
}
