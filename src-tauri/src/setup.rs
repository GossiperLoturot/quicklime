use crate::*;
use anyhow::Context;

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("setup app");

    utils::create_window_main(app.handle())?;
    setup_tray(app.handle())?;

    Ok(())
}

pub fn setup_plugin_clipboard() -> anyhow::Result<tauri::plugin::TauriPlugin<tauri::Wry>> {
    log::info!("setup plugin clipboard");

    Ok(tauri_plugin_clipboard_manager::init())
}

pub fn setup_plugin_global_shortcut() -> anyhow::Result<tauri::plugin::TauriPlugin<tauri::Wry>> {
    log::info!("setup plugin global_shortcut");

    let key = tauri_plugin_global_shortcut::Shortcut::new(
        Some(tauri_plugin_global_shortcut::Modifiers::CONTROL),
        tauri_plugin_global_shortcut::Code::Space,
    );

    // no occur panic in handle fn
    let handle = |app: &tauri::AppHandle,
                  _: &tauri_plugin_global_shortcut::Shortcut,
                  event: tauri_plugin_global_shortcut::ShortcutEvent| {
        if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            let Ok(position) = utils::get_window_center(app) else {
                log::error!("failed to get focus window center");
                return;
            };
            let Ok(()) = utils::locate_window_main(app, position) else {
                log::error!("failed to locate window main");
                return;
            };
            let Ok(()) = utils::show_window_main(app) else {
                log::error!("failed to show window main");
                return;
            };
        }
    };

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(key)
        .unwrap()
        .with_handler(handle)
        .build();
    Ok(plugin)
}

pub fn setup_tray(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let icon = app
        .default_window_icon()
        .context("default window icon is not fount")?
        .clone();

    let menu_show = tauri::menu::MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu_hide = tauri::menu::MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let menu_config = tauri::menu::MenuItem::with_id(app, "config", "Config", true, None::<&str>)?;
    let menu_quit = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu =
        tauri::menu::Menu::with_items(app, &[&menu_show, &menu_hide, &menu_config, &menu_quit])?;

    // no occur panic in handle fn
    let handle = |app: &tauri::AppHandle, event: tauri::menu::MenuEvent| {
        match event.id().as_ref() {
            "show" => {
                log::info!("send event: show window main");
                match utils::show_window_main(app) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("error occured {}", e);
                    }
                };
            }
            "hide" => {
                log::info!("send event: hide window main");
                match utils::hide_window_main(app) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("error occured {}", e);
                    }
                };
            }
            "config" => {
                log::info!("send event: setup window config");
                match utils::create_window_config(app) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("error occured {}", e);
                    }
                };
            }
            "quit" => {
                log::info!("send event: quit app");
                app.exit(0);
            }
            _ => {
                log::error!("invalid event id");
            }
        };
    };

    tauri::tray::TrayIconBuilder::new()
        .icon(icon)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(handle)
        .build(app)?;

    Ok(())
}

pub fn setup_window_event() -> anyhow::Result<impl Fn(&tauri::Window, &tauri::WindowEvent)> {
    // no occur panic in handle fn
    let handle = |window: &tauri::Window, event: &tauri::WindowEvent| {
        if let tauri::WindowEvent::Focused(false) = event {
            let app = tauri::Manager::app_handle(window);

            match utils::hide_window_main(app) {
                Ok(_) => {}
                Err(e) => log::error!("error occured {}", e),
            }
        }
    };
    Ok(handle)
}
