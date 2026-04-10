use std::sync::{Mutex};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW,
    HHOOK, MSLLHOOKSTRUCT, WH_MOUSE_LL, WH_KEYBOARD_LL,
    WM_XBUTTONDOWN, WM_XBUTTONUP, WM_KEYDOWN, KBDLLHOOKSTRUCT,
};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub struct HookConfig {
    pub key_next: u32,
    pub key_prev: u32,
    pub mouse_next: u32,
    pub mouse_prev: u32,
}

static HOOK_CALLBACK: Mutex<Option<Box<dyn Fn(bool) + Send>>> = Mutex::new(None);

// Atomics pour éviter tout lock dans les callbacks
static KEY_NEXT:   AtomicU32 = AtomicU32::new(0);
static KEY_PREV:   AtomicU32 = AtomicU32::new(0);
static MOUSE_NEXT: AtomicU32 = AtomicU32::new(2);
static MOUSE_PREV: AtomicU32 = AtomicU32::new(1);
static HOOK_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_enabled(enabled: bool) {
    HOOK_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn update_config(config: HookConfig) {
    KEY_NEXT.store(config.key_next, Ordering::Relaxed);
    KEY_PREV.store(config.key_prev, Ordering::Relaxed);
    MOUSE_NEXT.store(config.mouse_next, Ordering::Relaxed);
    MOUSE_PREV.store(config.mouse_prev, Ordering::Relaxed);
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
            let mouse_next = MOUSE_NEXT.load(Ordering::Relaxed);
            let mouse_prev = MOUSE_PREV.load(Ordering::Relaxed);

            if button == mouse_next || button == mouse_prev {
                if msg == WM_XBUTTONDOWN {
                    if let Ok(cb) = HOOK_CALLBACK.try_lock() {
                        if let Some(f) = cb.as_ref() {
                            f(button == mouse_next);
                        }
                    }
                }
                return LRESULT(1);
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
        let key_next = KEY_NEXT.load(Ordering::Relaxed);
        let key_prev = KEY_PREV.load(Ordering::Relaxed);

        if key_next > 0 && info.vkCode == key_next {
            if let Ok(cb) = HOOK_CALLBACK.try_lock() {
                if let Some(f) = cb.as_ref() { f(true); }
            }
        } else if key_prev > 0 && info.vkCode == key_prev {
            if let Ok(cb) = HOOK_CALLBACK.try_lock() {
                if let Some(f) = cb.as_ref() { f(false); }
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