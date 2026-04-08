#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod windows_api;
mod mouse_hook;

use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};
use windows_api::{detect_dofus_windows, GameWindow, SwitcherState};

static APP_HANDLE: std::sync::OnceLock<tauri::AppHandle> = std::sync::OnceLock::new();

struct AppState(Arc<Mutex<SwitcherState>>);

#[tauri::command]
fn detect_windows(state: State<AppState>) -> Vec<GameWindow> {
    let mut s = state.0.lock().unwrap();
    let found = detect_dofus_windows();
    s.accounts = found.clone();
    s.current_index = 0;
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

fn main() {
    let shared = Arc::new(Mutex::new(SwitcherState::new()));
    let hook_shared = Arc::clone(&shared);

    mouse_hook::install_mouse_hook(move |next| {
        let mut s = hook_shared.lock().unwrap();
        if next { s.switch_next(); } else { s.switch_prev(); }
        let index = s.current_index;
        if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit("switch", index);
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let _ = APP_HANDLE.set(app.handle().clone());
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
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au démarrage de Tauri");
}