use anyhow::Context;

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("setup app");

    setup_window_main(app.handle())?;
    setup_tray(app.handle())?;

    Ok(())
}

pub fn plugin_clipboard() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    log::info!("setup plugin clipboard");

    tauri_plugin_clipboard_manager::init()
}

pub fn plugin_global_shortcut() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    log::info!("setup plugin global_shortcut");

    let key = tauri_plugin_global_shortcut::Shortcut::new(
        Some(tauri_plugin_global_shortcut::Modifiers::CONTROL),
        tauri_plugin_global_shortcut::Code::Space,
    );

    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(key)
        .unwrap()
        .with_handler(move |app, _, event| {
            if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                let position = get_window_center().unwrap();
                locate_window_main(app, position).unwrap();
                show_window_main(app).unwrap();
            }
        })
        .build()
}

pub fn setup_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("setup window main");

    let url = tauri::WebviewUrl::App("main".into());
    tauri::WebviewWindowBuilder::new(app, "main", url)
        .title("Quicklime")
        .always_on_top(true)
        .closable(false)
        .decorations(false)
        .fullscreen(false)
        .maximizable(false)
        .minimizable(false)
        .resizable(false)
        .shadow(false)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    Ok(())
}

pub fn show_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("show window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;
    window.show()?;
    window.set_focus()?;

    Ok(())
}

pub fn hide_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("hide window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;
    window.hide()?;

    Ok(())
}

fn get_window_center() -> anyhow::Result<tauri::Position> {
    log::info!("get forground window center position");

    let mut lprect = windows::Win32::Foundation::RECT::default();

    unsafe {
        let hwnd = windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
        windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut lprect)
            .context("failed to get window rect")?;
    }

    let x = (lprect.right + lprect.left) as f64 / 2.0;
    let y = (lprect.top + lprect.bottom) as f64 / 2.0;
    Ok(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
}

fn locate_window_main(app: &tauri::AppHandle, position: tauri::Position) -> anyhow::Result<()> {
    const W_WIDTH: f64 = 1024.0;
    const W_HEIGHT: f64 = 512.0;

    log::info!("locate window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;

    let size = tauri::Size::Logical(tauri::LogicalSize::new(W_WIDTH, W_HEIGHT));
    window.set_size(size).unwrap();

    let tauri::Position::Logical(position) = position else {
        anyhow::bail!("position must be logical type")
    };
    let x = position.x - W_WIDTH / 2.0;
    let y = position.y - W_HEIGHT / 2.0;
    let position = tauri::Position::Logical(tauri::LogicalPosition::new(x, y));
    window.set_position(position).unwrap();
    Ok(())
}

pub fn setup_window_config(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let url = tauri::WebviewUrl::App("config".into());
    tauri::WebviewWindowBuilder::new(app, "config", url)
        .title("Quicklime")
        .build()?;

    Ok(())
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
    let handle = |app: &tauri::AppHandle, event: tauri::menu::MenuEvent| {
        let ret = match event.id().as_ref() {
            "show" => {
                log::info!("send event: show window main");
                show_window_main(app)
            }
            "hide" => {
                log::info!("send event: hide window main");
                hide_window_main(app)
            }
            "config" => {
                log::info!("send event: setup window config");
                setup_window_config(app)
            }
            "quit" => {
                log::info!("send event: quit app");
                app.exit(0);
                Ok(())
            }
            _ => Err(anyhow::anyhow!("invalid event id")),
        };
        match ret {
            Ok(()) => {}
            Err(e) => log::error!("tray menu handler: {:?}", e),
        }
    };

    tauri::tray::TrayIconBuilder::new()
        .icon(icon)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(handle)
        .build(app)?;

    Ok(())
}

pub fn on_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    if let tauri::WindowEvent::Focused(false) = event {
        window.hide().unwrap();
    }
}
