use serde::Serialize;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, EnumWindows, GetWindowTextW, GetWindowThreadProcessId,
    GetForegroundWindow, IsWindowVisible, SetForegroundWindow, ShowWindow, SW_RESTORE,
};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};

#[derive(Debug, Clone, Serialize)]
pub struct GameWindow {
    pub hwnd: isize,
    pub title: String,
    pub enabled: bool,
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows = &mut *(lparam.0 as *mut Vec<GameWindow>);

    if IsWindowVisible(hwnd).as_bool() {
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            let title = String::from_utf16_lossy(&buf[..len as usize]);
            if title == "Dofus" || title.contains("- Release") {
                windows.push(GameWindow {
                    hwnd: hwnd.0 as isize,
                    title,
                    enabled: true,
                });
            }
        }
    }
    BOOL(1)
}

pub fn detect_dofus_windows() -> Vec<GameWindow> {
    let mut windows: Vec<GameWindow> = Vec::new();
    unsafe {
        let ptr = &mut windows as *mut Vec<GameWindow>;
        let _ = EnumWindows(Some(enum_callback), LPARAM(ptr as isize));
    }
    windows
}

pub fn focus_window(hwnd: isize) {
    let hwnd = HWND(hwnd as *mut _);
    unsafe {
        let fg_hwnd = GetForegroundWindow();
        let fg_thread = GetWindowThreadProcessId(fg_hwnd, None);
        let current_thread = GetCurrentThreadId();
        let target_thread = GetWindowThreadProcessId(hwnd, None);

        let _ = AttachThreadInput(fg_thread, current_thread, true);
        let _ = AttachThreadInput(current_thread, target_thread, true);

        let _ = ShowWindow(hwnd, SW_RESTORE);
        let _ = BringWindowToTop(hwnd);
        let _ = SetForegroundWindow(hwnd);

        let _ = AttachThreadInput(fg_thread, current_thread, false);
        let _ = AttachThreadInput(current_thread, target_thread, false);
    }
}

pub struct SwitcherState {
    pub accounts: Vec<GameWindow>,
    pub current_index: usize,
}

impl SwitcherState {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            current_index: 0,
        }
    }

    pub fn active_accounts(&self) -> Vec<isize> {
        self.accounts
            .iter()
            .filter(|a| a.enabled)
            .map(|a| a.hwnd)
            .collect()
    }

    pub fn switch_next(&mut self) {
        let active = self.active_accounts();
        if active.is_empty() { return; }
        self.current_index = (self.current_index + 1) % active.len();
        focus_window(active[self.current_index]);
    }

    pub fn switch_prev(&mut self) {
        let active = self.active_accounts();
        if active.is_empty() { return; }
        let len = active.len();
        self.current_index = (self.current_index + len - 1) % len;
        focus_window(active[self.current_index]);
    }
}