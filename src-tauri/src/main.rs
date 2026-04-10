#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod windows_api;
mod mouse_hook;
mod overlay;
mod system;

use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};
use windows_api::{detect_dofus_windows, GameWindow, SwitcherState};

static APP_HANDLE: std::sync::OnceLock<tauri::AppHandle> = std::sync::OnceLock::new();

pub struct AppState(pub Arc<Mutex<SwitcherState>>);

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
fn toggle_account(state: State<AppState>, hwnd: isize, enabled: bool) {
    let mut s = state.0.lock().unwrap();
    if let Some(acc) = s.accounts.iter_mut().find(|a| a.hwnd == hwnd) {
        acc.enabled = enabled;
    }
}

#[tauri::command]
fn reorder_accounts(state: State<AppState>, hwnds: Vec<isize>) {
    let mut s = state.0.lock().unwrap();
    let mut reordered = Vec::new();
    for hwnd in &hwnds {
        if let Some(acc) = s.accounts.iter().find(|a| a.hwnd == *hwnd) {
            reordered.push(acc.clone());
        }
    }
    s.accounts = reordered;
    s.current_index = 0;
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
fn set_hook_enabled(enabled: bool) {
    mouse_hook::set_enabled(enabled);
}

#[tauri::command]
fn quit(handle: tauri::AppHandle) {
    handle.exit(0);
}

fn main() {
    let shared = Arc::new(Mutex::new(SwitcherState::new()));
    let hook_shared = Arc::clone(&shared);

    mouse_hook::install_mouse_hook(move |next| {
        let hwnd = {
            let mut s = hook_shared.lock().unwrap();
            if next { s.next_hwnd() } else { s.prev_hwnd() }
        };
        if let Some(hwnd) = hwnd {
            if let Some(handle) = APP_HANDLE.get() {
                let _ = handle.emit("switch", hwnd);
            }
            std::thread::spawn(move || {
                windows_api::focus_window(hwnd);
            });
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let _ = APP_HANDLE.set(app.handle().clone());
            overlay::open_overlay(app.handle());
            Ok(())
        })
        .manage(AppState(shared))
        .invoke_handler(tauri::generate_handler![
            detect_windows,
            toggle_account,
            reorder_accounts,
            update_hotkeys,
            focus_account,
            get_accounts,
            set_hook_enabled,
            quit,
            overlay::toggle_overlay,
            overlay::resize_overlay,
            overlay::show_main_window,
            system::get_app_paths,
            system::open_folder,
            system::uninstall_app,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au démarrage de Tauri");
}