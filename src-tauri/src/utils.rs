use crate::*;

use anyhow::Context;

pub fn create_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
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

pub fn create_window_config(app: &tauri::AppHandle) -> anyhow::Result<()> {
    let url = tauri::WebviewUrl::App("config".into());
    tauri::WebviewWindowBuilder::new(app, "config", url)
        .title("Quicklime")
        .min_inner_size(480.0, 270.0)
        .build()?;

    Ok(())
}

pub fn show_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
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

pub fn hide_window_main(app: &tauri::AppHandle) -> anyhow::Result<()> {
    log::info!("hide window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;
    if window.is_visible()? {
        window.hide()?;
        tauri::Emitter::emit(&window, "hide_window", ())?;
    }

    Ok(())
}

pub fn get_window_center(app: &tauri::AppHandle) -> anyhow::Result<tauri::Position> {
    log::info!("get forground window center position");

    let hwnd = unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() };

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

    let mut lprect = windows::Win32::Foundation::RECT::default();
    unsafe { windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut lprect) }
        .context("failed to get window rect")?;

    let x = (lprect.right + lprect.left) as f64 / 2.0;
    let y = (lprect.top + lprect.bottom) as f64 / 2.0;
    Ok(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
}

pub fn locate_window_main(app: &tauri::AppHandle, position: tauri::Position) -> anyhow::Result<()> {
    const W_WIDTH: f64 = 512.0;
    const W_HEIGHT: f64 = 512.0;

    log::info!("locate window main");

    let window =
        tauri::Manager::get_webview_window(app, "main").context("window main is not found")?;

    let size = tauri::Size::Logical(tauri::LogicalSize::new(W_WIDTH, W_HEIGHT));
    window.set_size(size)?;

    let tauri::Position::Logical(position) = position else {
        anyhow::bail!("position must be logical type")
    };
    let x = position.x - W_WIDTH / 2.0;
    let y = position.y - W_HEIGHT / 2.0;
    let position = tauri::Position::Logical(tauri::LogicalPosition::new(x, y));
    window.set_position(position)?;
    Ok(())
}

// PowerToys at https://github.com/microsoft/PowerToys/blob/9f008a65d604313159e1e83607e8240b0d49098d/src/modules/AdvancedPaste/AdvancedPaste/Helpers/ClipboardHelper.cs#L89
pub fn paste_clipboard(app: &tauri::AppHandle, text: &str) -> anyhow::Result<()> {
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

pub async fn request_processing(
    cache: &mut lru::LruCache<setup::Query, String>,
    input: &str,
    mode: usize,
    config: &setup::Config,
) -> anyhow::Result<String> {
    let query: setup::Query = (input.into(), mode);
    if let Some(output) = cache.get(&query) {
        log::info!("[cache hit] text: {}, mode: {}", input, mode);
        return Ok(output.into());
    }
    log::info!("[cache miss] text: {}, mode: {}", input, mode);

    let response = match config.llm {
        setup::LLM_CHATGPT => {
            let prompt = match mode {
                setup::MODE_TRANSLATION => serde_json::json!({
                    "model": "gpt-4o-mini",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional translation engine. Please translate the text into English without explanation."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence. I reply only the translated sentence, otherwise reply empty string."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                setup::MODE_POLISHING => serde_json::json!({
                    "model": "gpt-4o-mini",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional polishing engine. Please polish the text."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence. I reply only the polished sentence, otherwise reply empty string."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                setup::MODE_COMPLETION => serde_json::json!({
                    "model": "gpt-4o-mini",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional completion engine. Please complete the text."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                _ => return Err(anyhow::anyhow!("failed to create prompt")),
            };
            let client = tauri_plugin_http::reqwest::Client::new();
            client
                .post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(&config.token)
                .header("Content-Type", "application/json")
                .json(&prompt)
                .send()
                .await?
        }
        setup::LLM_GROK => {
            let prompt = match mode {
                setup::MODE_TRANSLATION => serde_json::json!({
                    "model": "grok-2-latest",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional translation engine. Please translate the text into English without explanation."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                setup::MODE_POLISHING => serde_json::json!({
                    "model": "grok-2-latest",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional polishing engine. Please polish the text."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                setup::MODE_COMPLETION => serde_json::json!({
                    "model": "grok-2-latest",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are a professional completion engine. Please complete the text."
                        },
                        {
                            "role": "assistant",
                            "content": "Yes, I understand. Please give me the sentence."
                        },
                        {
                            "role": "user",
                            "content": input
                        }
                    ]
                }),
                _ => return Err(anyhow::anyhow!("failed to create prompt")),
            };
            let client = tauri_plugin_http::reqwest::Client::new();
            client
                .post("https://api.x.ai/v1/chat/completions")
                .bearer_auth(&config.token)
                .header("Content-Type", "application/json")
                .json(&prompt)
                .send()
                .await?
        }
        _ => unreachable!(),
    };

    let data = response.json::<serde_json::Value>().await?;
    let extractor = jsonpath_rust::JsonPath::try_from("$.choices[*].message.content")?;
    let output: String = extractor
        .find(&data)
        .as_array()
        .and_then(|arr| arr.iter().flat_map(|item| item.as_str()).next())
        .unwrap_or("")
        .into();

    cache.put(query, output.clone());

    Ok(output)
}
