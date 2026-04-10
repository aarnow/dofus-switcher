use tauri::{Emitter, Manager, WebviewWindowBuilder, WebviewUrl};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
use windows::Win32::Graphics::Gdi::{
    MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use crate::AppState;
use tauri::State;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

static OVERLAY_VARIANT: Mutex<String> = Mutex::new(String::new());
static OVERLAY_W: AtomicU32 = AtomicU32::new(160);
static OVERLAY_H: AtomicU32 = AtomicU32::new(88);
static OVERLAY_X: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);
static OVERLAY_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(12);

#[tauri::command]
pub fn update_overlay_config(
    handle: tauri::AppHandle,
    variant: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
) {
    {
        let mut v = OVERLAY_VARIANT.lock().unwrap();
        *v = variant.clone();
    }
    OVERLAY_W.store(width, Ordering::Relaxed);
    OVERLAY_H.store(height, Ordering::Relaxed);
    OVERLAY_X.store(x, Ordering::Relaxed);
    OVERLAY_Y.store(y, Ordering::Relaxed);

    if let Some(w) = handle.get_webview_window("overlay") {
        let _ = w.close();
    }
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        if let Some(h) = crate::APP_HANDLE.get() {
            open_overlay(h);
        }
    });
}

#[tauri::command]
pub fn move_compact_overlay(handle: tauri::AppHandle, x: i32, y: i32) {
    OVERLAY_X.store(x, Ordering::Relaxed);
    OVERLAY_Y.store(y, Ordering::Relaxed);

    if let Some(w) = handle.get_webview_window("overlay") {
        let final_x = if x == -1 {
            let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
            let w_val = OVERLAY_W.load(Ordering::Relaxed) as i32;
            (screen_width - w_val) / 2
        } else { x };

        let _ = w.set_position(tauri::Position::Physical(
            tauri::PhysicalPosition { x: final_x, y }
        ));
    }
}

pub fn open_overlay(handle: &tauri::AppHandle) {
   let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
   let variant = OVERLAY_VARIANT.lock().unwrap().clone();
   let is_compact = variant == "compact";

   let overlay_w = if is_compact { OVERLAY_W.load(Ordering::Relaxed) as i32 } else { 160i32 };
   let overlay_h = if is_compact { OVERLAY_H.load(Ordering::Relaxed) as i32 } else { 88i32 };

   let saved_x = OVERLAY_X.load(Ordering::Relaxed);
   let saved_y = OVERLAY_Y.load(Ordering::Relaxed);

   let x = if !is_compact || saved_x == -1 {
       (screen_width / 2) - (overlay_w / 2)
   } else { saved_x };

   let y = if !is_compact { 12 } else { saved_y };

    #[cfg(debug_assertions)]
    let url = if is_compact {
        WebviewUrl::External("http://localhost:3000/overlay-compact".parse().unwrap())
    } else {
        WebviewUrl::External("http://localhost:3000/overlay".parse().unwrap())
    };

    #[cfg(not(debug_assertions))]
    let url = if is_compact {
        WebviewUrl::App("overlay-compact".into())
    } else {
        WebviewUrl::App("overlay".into())
    };

   let _ = WebviewWindowBuilder::new(handle, "overlay", url)
       .title("")
       .inner_size(overlay_w as f64, overlay_h as f64)
       .position(x as f64, y as f64)
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
pub fn resize_compact_overlay(handle: tauri::AppHandle, width: u32, height: u32) {
    OVERLAY_W.store(width, Ordering::Relaxed);
    OVERLAY_H.store(height, Ordering::Relaxed);

    if let Some(w) = handle.get_webview_window("overlay") {
        let _ = w.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width,
            height,
        }));
    }
}

#[tauri::command]
pub fn save_overlay_position(handle: tauri::AppHandle, x: i32, y: i32) {
    OVERLAY_X.store(x, Ordering::Relaxed);
    OVERLAY_Y.store(y, Ordering::Relaxed);
    // Sauvegarde dans le store via un event
    let _ = handle.emit("save-overlay-position", (x, y));
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