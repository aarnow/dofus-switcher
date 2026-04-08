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
fn set_hook_enabled(enabled: bool) {
    mouse_hook::set_enabled(enabled);
}

fn open_overlay(handle: &tauri::AppHandle) {
    eprintln!("Tentative d'ouverture de l'overlay...");

    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN};
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let overlay_w = 160i32;
    let overlay_h = 44i32;
    let x = (screen_width / 2) - (overlay_w / 2);
    let y = 12i32;

    let result = WebviewWindowBuilder::new(
        handle,
        "overlay",
        WebviewUrl::External("http://localhost:3000/overlay".parse().unwrap()),
    )
    .title("")
    .inner_size(overlay_w as f64, overlay_h as f64)
    .position(x as f64, y as f64)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .shadow(false)
    .devtools(true)
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
            set_hook_enabled
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au démarrage de Tauri");
}