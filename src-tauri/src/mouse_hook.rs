use std::sync::Mutex;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW,
    HHOOK, MSLLHOOKSTRUCT, WH_MOUSE_LL, WH_KEYBOARD_LL,
    WM_XBUTTONDOWN, WM_XBUTTONUP, WM_KEYDOWN, KBDLLHOOKSTRUCT,
};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct HookConfig {
    pub key_next: u32,
    pub key_prev: u32,
    pub mouse_next: u32,
    pub mouse_prev: u32,
}

static HOOK_CALLBACK: Mutex<Option<Box<dyn Fn(bool) + Send>>> = Mutex::new(None);
static HOOK_CONFIG: Mutex<HookConfig> = Mutex::new(HookConfig {
    key_next: 0,
    key_prev: 0,
    mouse_next: 2,
    mouse_prev: 1,
});
static HOOK_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_enabled(enabled: bool) {
    HOOK_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn update_config(config: HookConfig) {
    let mut c = HOOK_CONFIG.lock().unwrap();
    *c = config;
}

unsafe extern "system" fn mouse_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if !HOOK_ENABLED.load(Ordering::Relaxed) {
        return CallNextHookEx(HHOOK::default(), n_code, w_param, l_param);
    }

    if n_code >= 0 {
        let msg = w_param.0 as u32;
        let is_xbutton = msg == WM_XBUTTONDOWN || msg == WM_XBUTTONUP;

        if is_xbutton {
            let info = &*(l_param.0 as *const MSLLHOOKSTRUCT);
            let button = (info.mouseData >> 16) & 0xFFFF;
            if let Ok(cfg) = HOOK_CONFIG.lock() {
                if button == cfg.mouse_next || button == cfg.mouse_prev {
                    if msg == WM_XBUTTONDOWN {
                        if let Ok(cb) = HOOK_CALLBACK.lock() {
                            if let Some(f) = cb.as_ref() {
                                if button == cfg.mouse_next { f(true); }
                                else { f(false); }
                            }
                        }
                    }
                    return LRESULT(1);
                }
            }
        }
    }
    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

unsafe extern "system" fn keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if !HOOK_ENABLED.load(Ordering::Relaxed) {
        return CallNextHookEx(HHOOK::default(), n_code, w_param, l_param);
    }

    if n_code >= 0 && w_param.0 as u32 == WM_KEYDOWN {
        let info = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
        if let (Ok(cb), Ok(cfg)) = (HOOK_CALLBACK.lock(), HOOK_CONFIG.lock()) {
            if let Some(f) = cb.as_ref() {
                if info.vkCode == cfg.key_next { f(true); }
                else if info.vkCode == cfg.key_prev { f(false); }
            }
        }
    }
    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

pub fn install_mouse_hook(callback: impl Fn(bool) + Send + 'static) {
    {
        let mut cb = HOOK_CALLBACK.lock().unwrap();
        *cb = Some(Box::new(callback));
    }

    std::thread::spawn(|| unsafe {
        use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};

        let _mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), None, 0)
            .expect("Impossible d'installer le hook souris");

        let _keyboard_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0)
            .expect("Impossible d'installer le hook clavier");

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {}
    });
}