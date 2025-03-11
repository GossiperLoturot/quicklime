use crate::setup;

#[tauri::command]
pub fn on_confirm_input(app: tauri::AppHandle, text: &str) {
    setup::hide_window_main(&app).unwrap();
    paste_clipboard(&app, text).unwrap();
}

#[tauri::command]
pub fn on_exit_input(app: tauri::AppHandle) {
    setup::hide_window_main(&app).unwrap();
}

#[tauri::command]
pub fn on_change_input(app: tauri::AppHandle, text: &str) {
    todo!()
}

// PowerToys at https://github.com/microsoft/PowerToys/blob/9f008a65d604313159e1e83607e8240b0d49098d/src/modules/AdvancedPaste/AdvancedPaste/Helpers/ClipboardHelper.cs#L89
pub fn paste_clipboard(app: &tauri::AppHandle, text: &str) -> anyhow::Result<()> {
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

    unsafe {
        let mut pinput = windows::Win32::UI::Input::KeyboardAndMouse::INPUT::default();
        pinput.r#type = windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD;
        pinput.Anonymous.ki.wVk = key_code;
        pinput.Anonymous.ki.dwFlags = flags;
        pinput.Anonymous.ki.dwExtraInfo = EXTRA_INFO;
        let cbsize = std::mem::size_of_val(&pinput) as i32;
        windows::Win32::UI::Input::KeyboardAndMouse::SendInput(&[pinput], cbsize);
    }
}
