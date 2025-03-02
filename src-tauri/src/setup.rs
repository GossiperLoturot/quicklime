use anyhow::Context;

pub fn setup<R>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>>
where 
    R: tauri::Runtime
{
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto
    )?;

    setup_window_main(app.handle())?;
    setup_tray(app.handle())?;

    Ok(())
}

pub fn setup_window_main<R>(app: &tauri::AppHandle<R>) -> anyhow::Result<()>
where 
    R: tauri::Runtime
{
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

pub fn show_window_main<R>(app: &tauri::AppHandle<R>) -> anyhow::Result<()>
where 
    R: tauri::Runtime
{
    let window = tauri::Manager::get_webview_window(app, "main")
        .context("window main is not found")?;
    window.show()?;

    Ok(())
}

pub fn hide_window_main<R>(app: &tauri::AppHandle<R>) -> anyhow::Result<()>
where 
    R: tauri::Runtime
{
    let window = tauri::Manager::get_webview_window(app, "main")
        .context("window main is not found")?;
    window.hide()?;

    Ok(())
}

pub fn setup_window_config<R>(app: &tauri::AppHandle<R>) -> anyhow::Result<()>
where 
    R: tauri::Runtime
{
    let url = tauri::WebviewUrl::App("config".into());
    tauri::WebviewWindowBuilder::new(app, "config", url)
        .title("Quicklime")
        .build()?;

    Ok(())
}

pub fn setup_tray<R>(app: &tauri::AppHandle<R>) -> anyhow::Result<()>
where 
    R: tauri::Runtime
{
    let icon = app.default_window_icon()
        .context("default window icon is not fount")?
        .clone();

    let menu_show = tauri::menu::MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu_hide = tauri::menu::MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let menu_config = tauri::menu::MenuItem::with_id(app, "config", "Config", true, None::<&str>)?;
    let menu_quit = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = tauri::menu::Menu::with_items(app, &[
        &menu_show,
        &menu_hide,
        &menu_config,
        &menu_quit,
    ])?;
    let handle = |app: &tauri::AppHandle<R>, event: tauri::menu::MenuEvent| {
        let ret = match event.id().as_ref() {
            "show" => {
                log::info!("show main window");
                show_window_main(app)
            },
            "hide" => {
                log::info!("hide main window");
                hide_window_main(app)
            },
            "config" => {
                log::info!("create config window");
                setup_window_config(app)
            },
            "quit" => {
                log::info!("quit app");
                app.exit(0);
                Ok(())
            },
            _ => Err(anyhow::anyhow!("invalid event id")),
        };
        match ret {
            Ok(()) => {}
            Err(e) => log::error!("tray menu handler: {:?}", e)
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

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
