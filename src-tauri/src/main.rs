#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod windows_api;
mod mouse_hook;

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, State, WebviewWindowBuilder, WebviewUrl};
use windows_api::{detect_dofus_windows, GameWindow, SwitcherState};

static APP_HANDLE: std::sync::OnceLock<tauri::AppHandle> = std::sync::OnceLock::new();

struct AppState(Arc<Mutex<SwitcherState>>);

#[tauri::command]
fn detect_windows(state: State<AppState>) -> Vec<GameWindow> {
    let mut s = state.0.lock().unwrap();
    let found = detect_dofus_windows();
    s.accounts = found.clone();
    s.current_index = 0;
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit("accounts-updated", found.clone());
    }
    found
}

#[tauri::command]
fn switch_next(state: State<AppState>) {
    let mut s = state.0.lock().unwrap();
    s.switch_next();
}

#[tauri::command]
fn switch_prev(state: State<AppState>) {
    let mut s = state.0.lock().unwrap();
    s.switch_prev();
}

#[tauri::command]
fn toggle_account(state: State<AppState>, hwnd: isize, enabled: bool) {
    let mut s = state.0.lock().unwrap();
    if let Some(acc) = s.accounts.iter_mut().find(|a| a.hwnd == hwnd) {
        acc.enabled = enabled;
    }
}

#[tauri::command]
fn update_hotkeys(key_next: u32, key_prev: u32, mouse_next: u32, mouse_prev: u32) {
    mouse_hook::update_config(mouse_hook::HookConfig {
        key_next,
        key_prev,
        mouse_next,
        mouse_prev,
    });
}

#[tauri::command]
fn focus_account(_state: State<AppState>, hwnd: isize) {
    windows_api::focus_window(hwnd);
}

#[tauri::command]
fn get_accounts(state: State<AppState>) -> Vec<GameWindow> {
    state.0.lock().unwrap().accounts.clone()
}

#[tauri::command]
fn close_pie_menu() {
    if let Some(handle) = APP_HANDLE.get() {
        if let Some(w) = handle.get_webview_window("pie-menu") {
            let _ = w.close();
        }
    }
}

#[tauri::command]
fn toggle_overlay(state: State<AppState>, handle: tauri::AppHandle, enabled: bool) {
    if enabled {
        let accounts = state.0.lock().unwrap().accounts.clone();
                std::thread::spawn(move || {
                    // Attend que la fenêtre précédente soit bien fermée
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    if let Some(h) = APP_HANDLE.get() {
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
fn resize_overlay(handle: tauri::AppHandle, count: u32) {
    if let Some(w) = handle.get_webview_window("overlay") {
        let item_w = 74i32;
        let gap = 8i32;
        let btn_w = 56i32; // bouton 48px + gap
        let padding = 16i32;
        let overlay_w = btn_w + (item_w * count as i32) + (gap * count as i32) + padding;
        let overlay_h = 88i32;

        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
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
fn set_hook_enabled(enabled: bool) {
    mouse_hook::set_enabled(enabled);
}

#[tauri::command]
fn quit(handle: tauri::AppHandle) {
    handle.exit(0);
}

#[tauri::command]
fn show_main_window(state: State<AppState>, handle: tauri::AppHandle) {
    use windows::Win32::Graphics::Gdi::{
        MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };

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
                    let win_h = 600i32;
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

#[tauri::command]
fn uninstall_app(handle: tauri::AppHandle) {
    let app_data = std::env::var("APPDATA").unwrap_or_default();
    let config_dir = format!("{}\\DofusSwitcher", app_data);
    let _ = std::fs::remove_dir_all(config_dir);

    // Lance le désinstalleur Windows si disponible
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let uninstaller = dir.join("..\\unins000.exe");
            if uninstaller.exists() {
                let _ = std::process::Command::new(uninstaller).spawn();
            }
        }
    }
    handle.exit(0);
}

#[tauri::command]
fn get_app_paths() -> String {
    let app_data = std::env::var("APPDATA").unwrap_or_default();
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let config_path = format!("{}\\com.dofusswitcher.app\\settings.json", app_data);
    format!("{}|{}", exe_path, config_path)
}

#[tauri::command]
fn open_folder(folder_type: String) {
    let path = match folder_type.as_str() {
        "app" => std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_string_lossy().to_string()))
            .unwrap_or_default(),
        "config" => {
            let app_data = std::env::var("APPDATA").unwrap_or_default();
            format!("{}\\com.dofusswitcher.app", app_data)
        },
        _ => return,
    };
    let _ = std::process::Command::new("cmd")
        .args(["/c", "start", "", &path])
        .spawn();
}

fn open_overlay(handle: &tauri::AppHandle) {
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let overlay_w = 160i32;
    let overlay_h = 88i32;
    let x = (screen_width / 2) - (overlay_w / 2);

    let _ = WebviewWindowBuilder::new(
        handle,
        "overlay",
        WebviewUrl::External("http://localhost:3000/overlay".parse().unwrap()),
    )
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

fn main() {
    let shared = Arc::new(Mutex::new(SwitcherState::new()));
    let hook_shared = Arc::clone(&shared);

    mouse_hook::install_mouse_hook(move |next| {
        let mut s = hook_shared.lock().unwrap();
        if next { s.switch_next(); } else { s.switch_prev(); }
        let active = s.active_accounts();
        if let Some(&hwnd) = active.get(s.current_index) {
            if let Some(handle) = APP_HANDLE.get() {
                let _ = handle.emit("switch", hwnd);
            }
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let _ = APP_HANDLE.set(app.handle().clone());
            open_overlay(app.handle());
            Ok(())
        })
        .manage(AppState(shared))
        .invoke_handler(tauri::generate_handler![
            detect_windows,
            switch_next,
            switch_prev,
            toggle_account,
            update_hotkeys,
            focus_account,
            get_accounts,
            close_pie_menu,
            toggle_overlay,
            resize_overlay,
            set_hook_enabled,
            show_main_window,
            get_app_paths,
            uninstall_app,
            open_folder,
            quit
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au démarrage de Tauri");
}