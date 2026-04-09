#[tauri::command]
pub fn get_app_paths() -> String {
    let app_data = std::env::var("APPDATA").unwrap_or_default();
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let config_path = format!("{}\\com.dofusswitcher.app\\settings.json", app_data);
    format!("{}|{}", exe_path, config_path)
}

#[tauri::command]
pub fn open_folder(folder_type: String) {
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

#[tauri::command]
pub fn uninstall_app(handle: tauri::AppHandle) {
    let app_data = std::env::var("APPDATA").unwrap_or_default();
    let config_dir = format!("{}\\com.dofusswitcher.app", app_data);
    let _ = std::fs::remove_dir_all(config_dir);

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