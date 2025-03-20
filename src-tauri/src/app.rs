use anyhow::Context;

use crate::*;

pub const MODE_TRANSLATION: usize = 0;
pub const MODE_POLISHING: usize = 1;
pub const MODE_COMPLETION: usize = 2;

pub const LLM_CHATGPT: usize = 0;
pub const LLM_GROK: usize = 1;

pub type Query = (String, usize);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub llm: usize,
    pub token: String,
    pub rate: f64,
    pub language: String,
    pub cache_size: usize,
}

struct AppConfig {
    config: tauri::async_runtime::Mutex<Config>,
}

struct AppChannel {
    tx_input: crossbeam_channel::Sender<Query>,
    _th_input: tauri::async_runtime::JoinHandle<()>,
}

struct AppCache {
    cache: tauri::async_runtime::Mutex<lru::LruCache<Query, String>>,
}

pub fn setup_plugin_clipboard() -> anyhow::Result<impl tauri::plugin::Plugin<tauri::Wry>> {
    log::info!("setup plugin clipboard");

    Ok(tauri_plugin_clipboard_manager::init())
}

pub fn setup_plugin_global_shortcut() -> anyhow::Result<impl tauri::plugin::Plugin<tauri::Wry>> {
    log::info!("setup plugin global_shortcut");

    let key = tauri_plugin_global_shortcut::Shortcut::new(
        Some(tauri_plugin_global_shortcut::Modifiers::CONTROL),
        tauri_plugin_global_shortcut::Code::Space,
    );

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(key)
        .unwrap()
        .with_handler(global_shortcut_handle)
        .build();
    Ok(plugin)
}

// no occur panic in handle fn
fn global_shortcut_handle(
    app: &tauri::AppHandle,
    _: &tauri_plugin_global_shortcut::Shortcut,
    event: tauri_plugin_global_shortcut::ShortcutEvent,
) {
    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
        match pop_window_main(app) {
            Ok(_) => {}
            Err(e) => {
                log::error!("error occured {}", e);
                return;
            }
        };
    }
}

fn pop_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("get forground window center position");

    let hwnd = unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() };

    // check focus window
    if let Some(window) = tauri::Manager::get_webview_window(app, "main") {
        let slf_hwnd = window.hwnd()?;
        if hwnd == slf_hwnd {
            return Err(anyhow::anyhow!("focus window must be no window main"));
        }
    }
    if let Some(window) = tauri::Manager::get_webview_window(app, "config") {
        let slf_hwnd = window.hwnd()?;
        if hwnd == slf_hwnd {
            return Err(anyhow::anyhow!("focus window must be no window config"));
        }
    }

    log::info!("get focus window center");
    let mut lprect = windows::Win32::Foundation::RECT::default();
    unsafe { windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut lprect) }
        .context("failed to get window rect")?;
    let x = (lprect.right + lprect.left) as f64 / 2.0;
    let y = (lprect.top + lprect.bottom) as f64 / 2.0;

    const W_WIDTH: f64 = 512.0;
    const W_HEIGHT: f64 = 512.0;
    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;

    log::info!("locate window main");
    let x = x - W_WIDTH / 2.0;
    let y = y - W_HEIGHT / 2.0;
    let position = tauri::Position::Logical(tauri::LogicalPosition::new(x, y));
    window.set_position(position)?;

    log::info!("resize window main");
    let size = tauri::Size::Logical(tauri::LogicalSize::new(W_WIDTH, W_HEIGHT));
    window.set_size(size)?;

    show_window_main(app)?;

    Ok(())
}

fn show_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("show window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;
    if !window.is_visible()? {
        window.show()?;
        window.set_focus()?;
        tauri::Emitter::emit(&window, "show_window", ())?;
    }

    Ok(())
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("setup app");

    create_window_main(app.handle())?;

    setup_config(app.handle())?;
    setup_cache(app.handle())?;
    setup_channel(app.handle())?;
    setup_tray(app.handle())?;

    Ok(())
}

fn create_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
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
        .focused(false)
        .transparent(true)
        .build()?;

    Ok(())
}

fn setup_config(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let filepath =
        tauri::Manager::path(app).resolve("config.json", tauri::path::BaseDirectory::AppConfig)?;

    let config = std::fs::File::open(filepath)
        .ok()
        .and_then(|rdr| serde_json::from_reader::<_, Config>(rdr).ok())
        .unwrap_or_else(|| Config {
            llm: LLM_CHATGPT,
            token: Default::default(),
            rate: 0.5,
            language: "English".into(),
            cache_size: 1024,
        });

    let config = tauri::async_runtime::Mutex::new(config);
    let state = AppConfig { config };
    tauri::Manager::manage(app, state);

    Ok(())
}

fn setup_cache(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let state = tauri::Manager::state::<AppConfig>(app);
    let config = state.config.blocking_lock();

    let cache_size = std::num::NonZeroUsize::try_from(config.cache_size)?;
    let cache = lru::LruCache::new(cache_size);

    let cache = tauri::async_runtime::Mutex::new(cache);
    let state = AppCache { cache };
    tauri::Manager::manage(app, state);

    Ok(())
}

fn setup_channel(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let state = tauri::Manager::state::<AppConfig>(app);
    let config = state.config.blocking_lock();

    let min_duration = std::time::Duration::from_secs_f64(config.rate);

    let (tx_input, rx_input) = crossbeam_channel::unbounded::<Query>();

    let app_clone = app.clone();
    let _th_input = tauri::async_runtime::spawn(async move {
        loop {
            let Ok((input, mode)) = rx_input.recv() else {
                continue;
            };

            if !rx_input.is_empty() {
                continue;
            }

            log::info!("input: {}", input);
            let instant = std::time::Instant::now();

            let state = tauri::Manager::state::<AppCache>(&app_clone);
            let mut cache = state.cache.lock().await;

            let state = tauri::Manager::state::<AppConfig>(&app_clone);
            let config = state.config.lock().await;

            let input = input.trim();
            let output = match llm::request_llm(&mut cache, input, mode, &config).await {
                Ok(output) => output,
                Err(e) => {
                    log::error!("error occured {}", e);
                    continue;
                }
            };
            match tauri::Emitter::emit(&app_clone, "update_output", output) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("error occured {}", e);
                    continue;
                }
            }

            let duration = min_duration.saturating_sub(instant.elapsed());
            tokio::time::sleep(duration).await;
        }
    });

    let state = AppChannel {
        tx_input,
        _th_input,
    };
    tauri::Manager::manage(app, state);

    Ok(())
}

fn setup_tray(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let icon = app
        .default_window_icon()
        .context("default window icon is not fount")?
        .clone();

    let menu_show = tauri::menu::MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu_config = tauri::menu::MenuItem::with_id(app, "config", "Config", true, None::<&str>)?;
    let menu_quit = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = tauri::menu::Menu::with_items(app, &[&menu_show, &menu_config, &menu_quit])?;

    tauri::tray::TrayIconBuilder::new()
        .icon(icon)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(tray_handle)
        .build(app)?;

    Ok(())
}

// no occur panic in handle fn
fn tray_handle(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "show" => {
            log::info!("send event: show window main");
            match show_window_main(app) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("error occured {}", e);
                }
            };
        }
        "config" => {
            log::info!("send event: setup window config");
            match create_window_config(app) {
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
}

fn create_window_config(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let url = tauri::WebviewUrl::App("config".into());
    tauri::WebviewWindowBuilder::new(app, "config", url)
        .title("Quicklime")
        .min_inner_size(480.0, 270.0)
        .build()?;

    Ok(())
}

pub fn setup_window_event_handle() -> anyhow::Result<impl Fn(&tauri::Window, &tauri::WindowEvent)> {
    // no occur panic in handle fn
    let handle = |window: &tauri::Window, event: &tauri::WindowEvent| {
        if let tauri::WindowEvent::Focused(false) = event {
            let app = tauri::Manager::app_handle(window);

            match hide_window_main(app) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("error occured {}", e);
                    return;
                }
            }
        }
    };
    Ok(handle)
}

fn hide_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("hide window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;
    if window.is_visible()? {
        window.hide()?;
        tauri::Emitter::emit(&window, "hide_window", ())?;
    }

    Ok(())
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_confirm_input(app: tauri::AppHandle, input: String) {
    log::info!("confirm input {}", input);

    match confirm_input(&app, input) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

fn confirm_input(app: &tauri::AppHandle, input: String) -> anyhow::Result<()> {
    hide_window_main(app)?;
    paste_clipboard(app, &input)?;
    Ok(())
}

// PowerToys at https://github.com/microsoft/PowerToys/blob/9f008a65d604313159e1e83607e8240b0d49098d/src/modules/AdvancedPaste/AdvancedPaste/Helpers/ClipboardHelper.cs#L89
fn paste_clipboard(app: &tauri::AppHandle, text: &str) -> anyhow::Result<()> {
    log::info!("paste clipboard {}", text);

    tauri_plugin_clipboard_manager::ClipboardExt::clipboard(app).write_text(text)?;

    // key up modifier keys
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    send_input(VK_LCONTROL, KEYEVENTF_KEYUP);
    send_input(VK_RCONTROL, KEYEVENTF_KEYUP);
    send_input(VK_LWIN, KEYEVENTF_KEYUP);
    send_input(VK_RWIN, KEYEVENTF_KEYUP);
    send_input(VK_LSHIFT, KEYEVENTF_KEYUP);
    send_input(VK_RSHIFT, KEYEVENTF_KEYUP);
    send_input(VK_LMENU, KEYEVENTF_KEYUP);
    send_input(VK_RMENU, KEYEVENTF_KEYUP);
    // send ctrl v
    send_input(VK_CONTROL, Default::default());
    send_input(VK_V, Default::default());
    send_input(VK_V, KEYEVENTF_KEYUP);
    send_input(VK_CONTROL, KEYEVENTF_KEYUP);

    Ok(())
}

// SendInput at https://learn.microsoft.com/ja-jp/windows/win32/api/winuser/nf-winuser-sendinput
fn send_input(
    key_code: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
    flags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS,
) {
    const EXTRA_INFO: usize = 0x5555;

    let mut pinput = windows::Win32::UI::Input::KeyboardAndMouse::INPUT::default();
    pinput.r#type = windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD;
    pinput.Anonymous.ki.wVk = key_code;
    pinput.Anonymous.ki.dwFlags = flags;
    pinput.Anonymous.ki.dwExtraInfo = EXTRA_INFO;
    let cbsize = std::mem::size_of_val(&pinput) as i32;
    unsafe { windows::Win32::UI::Input::KeyboardAndMouse::SendInput(&[pinput], cbsize) };
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_exit_input(app: tauri::AppHandle) {
    log::info!("exit input");

    match exit_input(&app) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

fn exit_input(app: &tauri::AppHandle) -> anyhow::Result<()> {
    hide_window_main(app)?;
    Ok(())
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_change_input(app: tauri::AppHandle, input: String, mode: usize) {
    log::info!("change input");

    match change_input(&app, input, mode) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

fn change_input(app: &tauri::AppHandle, input: String, mode: usize) -> anyhow::Result<()> {
    let state = tauri::Manager::state::<AppChannel>(app);
    state.tx_input.send((input, mode))?;
    Ok(())
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_change_config(app: tauri::AppHandle, config: Config) {
    log::info!("change config {:?}", config);

    match change_config(&app, config) {
        Ok(_) => {}
        Err(e) => {
            log::error!("error occured {}", e);
            return;
        }
    }
}

fn change_config(app: &tauri::AppHandle, config: Config) -> anyhow::Result<()> {
    let state = tauri::Manager::state::<AppConfig>(app);
    *state.config.blocking_lock() = config.clone();

    let path = tauri::Manager::path(app).app_config_dir()?;
    std::fs::create_dir_all(&path)?;

    let filepath = path.join("config.json");
    let writer = std::fs::File::create(&filepath)?;
    serde_json::to_writer(writer, &config)?;
    log::info!("save config {:?}", filepath);

    // reset cache
    setup_cache(app)?;

    Ok(())
}

// no occur panic in handle fn
#[tauri::command]
pub fn on_get_config(app: tauri::AppHandle) -> Option<Config> {
    log::info!("get config");

    let config = match get_config(&app) {
        Ok(config) => config,
        Err(e) => {
            log::error!("error occured {}", e);
            return None;
        }
    };

    Some(config)
}

fn get_config(app: &tauri::AppHandle) -> anyhow::Result<Config> {
    let state = tauri::Manager::state::<AppConfig>(app);
    let config = state.config.blocking_lock().clone();
    Ok(config)
}
