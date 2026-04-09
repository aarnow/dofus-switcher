use tauri::{Emitter, Manager, WebviewWindowBuilder, WebviewUrl};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
use windows::Win32::Graphics::Gdi::{
    MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use crate::AppState;
use tauri::State;

pub fn open_overlay(handle: &tauri::AppHandle) {
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let overlay_w = 160i32;
    let overlay_h = 88i32;
    let x = (screen_width / 2) - (overlay_w / 2);

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External("http://localhost:3000/overlay".parse().unwrap());

    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("overlay".into());

    let _ = WebviewWindowBuilder::new(handle, "overlay", url)
        .title("")
        .inner_size(overlay_w as f64, overlay_h as f64)
        .position(x as f64, 12.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .shadow(false)
        .visible(true)
        .build();
}

#[tauri::command]
pub fn toggle_overlay(state: State<AppState>, handle: tauri::AppHandle, enabled: bool) {
    if enabled {
        let accounts = state.0.lock().unwrap().accounts.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(300));
            if let Some(h) = crate::APP_HANDLE.get() {
                open_overlay(h);
                std::thread::sleep(std::time::Duration::from_millis(800));
                let _ = h.emit("accounts-updated", accounts);
            }
        });
    } else {
        if let Some(w) = handle.get_webview_window("overlay") {
            let _ = w.close();
        }
    }
}

#[tauri::command]
pub fn resize_overlay(handle: tauri::AppHandle, count: u32) {
    if let Some(w) = handle.get_webview_window("overlay") {
        let item_w = 74i32;
        let gap = 8i32;
        let btn_w = 56i32;
        let padding = 16i32;
        let overlay_w = btn_w + (item_w * count as i32) + (gap * count as i32) + padding;
        let overlay_h = 88i32;

        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let x = (screen_width / 2) - (overlay_w / 2);

        let _ = w.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: overlay_w as u32,
            height: overlay_h as u32,
        }));
        let _ = w.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x,
            y: 12,
        }));
    }
}

#[tauri::command]
pub fn show_main_window(state: State<AppState>, handle: tauri::AppHandle) {
    if let Some(w) = handle.get_webview_window("main") {
        let s = state.0.lock().unwrap();
        let active = s.active_accounts();

        if let Some(&hwnd) = active.get(s.current_index) {
            unsafe {
                let hwnd_win = windows::Win32::Foundation::HWND(hwnd as *mut _);
                let monitor = MonitorFromWindow(hwnd_win, MONITOR_DEFAULTTONEAREST);

                let mut info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };

                if GetMonitorInfoW(monitor, &mut info).as_bool() {
                    let mx = info.rcWork.left;
                    let my = info.rcWork.top;
                    let mw = info.rcWork.right - info.rcWork.left;
                    let mh = info.rcWork.bottom - info.rcWork.top;
                    let win_w = 800i32;
                    let win_h = 650i32;
                    let x = mx + (mw - win_w) / 2;
                    let y = my + (mh - win_h) / 2;
                    let _ = w.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x, y }
                    ));
                }
            }
        } else {
            let _ = w.center();
        }

        let _ = w.show();
        let _ = w.set_focus();
    }
}