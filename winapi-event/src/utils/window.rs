use lazy_static::lazy_static;
use std::cell::RefCell;
use std::ptr;
use std::sync::Arc;
use tokio::sync::Mutex;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HHOOK, HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, ClipCursor, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    GetClientRect, GetKeyboardState, GetMessageW, GetSystemMetrics, LoadCursorW, PostQuitMessage,
    RegisterClassW, SetWindowsHookExW, ShowCursor, ShowWindow, ToUnicode, TranslateMessage,
    UnhookWindowsHookEx, CB_GETCOMBOBOXINFO, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    EVENT_SYSTEM_CAPTUREEND, GET_WHEEL_DELTA_WPARAM, GET_XBUTTON_WPARAM, HWND_DESKTOP, IDC_ARROW,
    KBDLLHOOKSTRUCT, MSG, PBT_APMBATTERYLOW, SC_KEYMENU, SC_MONITORPOWER, SC_SCREENSAVE,
    SM_CXSCREEN, SM_CYSCREEN, SPI_SETACTIVEWINDOWTRACKING, SW_SHOW, VK_ESCAPE, VK_LMENU, VK_LWIN,
    VK_MENU, VK_RWIN, VK_TAB, WH_KEYBOARD_LL, WM_DESTROY, WM_DISPLAYCHANGE, WM_KEYDOWN, WM_KEYUP,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
    WM_SYSCOMMAND, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TABLET_LAST, WM_XBUTTONDOWN, WM_XBUTTONUP,
    WNDCLASSW, WS_POPUP, XBUTTON1, XBUTTON2, WM_TABLET_FIRST
};

#[derive(Debug, Clone)]
pub struct Window {
    pub off: Arc<Mutex<bool>>,
    pub hook: Arc<RefCell<Option<HHOOK>>>
}

impl Window {
    pub fn new() -> Arc<Self> {
        Arc::new(Window {
            off: Arc::new(Mutex::new(true)),
            hook: Arc::new(RefCell::new(None))
        })
    }
}

thread_local! {
    static HOOK: RefCell<Option<HHOOK>> = RefCell::new(None);
}

#[cfg(target_os = "windows")]
impl Window {
    unsafe extern "system" fn keyboard_hook(
        code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        // println!("Keyboard hook triggered! 00000"); // ดูว่าฟังก์ชันทำงานหรือไม่
        if code >= 0 {
            // println!("Keyboard hook triggered!"); // ดูว่าฟังก์ชันทำงานหรือไม่
            let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
            if w_param as u32 == WM_SYSKEYDOWN {
                // println!("Key down detected: vkCode = {}", kb_struct.vkCode);
                if kb_struct.vkCode == VK_LMENU as u32 {
                    // println!("Alt + Tab pressed!");
                    // Block Alt + Tab by not calling CallNextHookEx
                    return 1;
                }
            }
        }
        CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
    }

    fn is_alt_pressed() -> bool {
        unsafe {
            let alt_state = winapi::um::winuser::GetAsyncKeyState(VK_MENU as i32) as i32;
            let is_pressed = (alt_state & 0x8000) != 0;
            println!("Alt state: {}, is_pressed: {}", alt_state, is_pressed);
            is_pressed
        }
    }

    pub fn set_keyboard_hook() {
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(Self::keyboard_hook),
                GetModuleHandleW(ptr::null()),
                0,
            );

            if hook.is_null() {
                panic!("Failed to set hook");
            }
            HOOK.with(|hook_cell| {
                *hook_cell.borrow_mut() = Some(hook); // เปลี่ยนค่าใน RefCell
            });
        }
    }

    pub fn unset_keyboard_hook(hwnd: &HWND) {
        unsafe {
            HOOK.with(|hook_cell| {
                if let Some(hook) = *hook_cell.borrow() {
                    UnhookWindowsHookEx(hook as _);
                    *hook_cell.borrow_mut() = None; // ล้างค่า
                }
            });
            Self::destroy(hwnd);
        }
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                ClipCursor(ptr::null());
                ShowCursor(BOOL::from(true)); // แสดงเคอร์เซอร์อีกครั้ง
                PostQuitMessage(0);
                0
            }
            WM_SYSKEYDOWN | WM_DISPLAYCHANGE => {
                println!("Alt + Tab pressed!");
                0
            }
            _ => DefWindowProcW(hwnd, msg, w_param, l_param),
        }
    }

    fn to_string(value: &str) -> Vec<u16> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(value)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }

    pub unsafe fn create_window() -> HWND {
        let class_name = Self::to_string("MyClass");
        let h_instance = GetModuleHandleW(ptr::null());
        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            hInstance: h_instance,
            lpfnWndProc: Some(Self::window_proc),
            lpszClassName: class_name.as_ptr(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            ..std::mem::zeroed()
        };

        RegisterClassW(&wnd_class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            Self::to_string("MyClass").as_ptr(),
            WS_POPUP,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );
        hwnd
    }

    pub unsafe fn show_window(hwnd: &HWND) -> BOOL {
        ShowWindow(*hwnd, SW_SHOW)
    }

    pub unsafe fn show_cursor(active: bool) -> c_int {
        ShowCursor(BOOL::from(active))
    }

    pub unsafe fn get_rect(hwnd: &HWND) -> RECT {
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(*hwnd, &mut rect);
        rect
    }

    pub unsafe fn lock_cursor(rect: &RECT) -> BOOL {
        ClipCursor(rect)
    }

    pub unsafe fn get_message(msg: &mut MSG) -> BOOL {
        GetMessageW(msg, ptr::null_mut(), 0, 0)
    }

    pub fn event() {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while Self::get_message(&mut msg) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                // println!("Mouse msg: {}", msg.message);
                match msg.message {
                    // WM_MOUSEMOVE => {
                    //     println!("Mouse moved to position: ({}, {})", msg.pt.x, msg.pt.y);
                    // }
                    // WM_LBUTTONDOWN => {
                    //     println!("Mouse left button pressed");
                    // }
                    // WM_LBUTTONUP => {
                    //     println!("Mouse left button released");
                    // }
                    // WM_RBUTTONDOWN => {
                    //     println!("Mouse right button pressed");
                    // }
                    // WM_RBUTTONUP => {
                    //     println!("Mouse right button released");
                    // }
                    // WM_MOUSEWHEEL => {
                    //     let delta = GET_WHEEL_DELTA_WPARAM(msg.wParam) as i16;
                    //     if delta > 0 {
                    //         println!("Mouse wheel scrolled up: {}", delta);
                    //     } else {
                    //         println!("Mouse wheel scrolled down: {}", delta);
                    //     }
                    // }
                    // WM_XBUTTONUP => {
                    //     let xbutton = GET_XBUTTON_WPARAM(msg.wParam);
                    //     match xbutton {
                    //         XBUTTON1 => println!("XButton1 (Back) pressed"),
                    //         XBUTTON2 => println!("XButton2 (Forward) pressed"),
                    //         _ => println!("Unknown XButton pressed"),
                    //     }
                    // }
                    WM_KEYDOWN => {
                        // println!("Key pressed: {}", msg.wParam);
                        if let Some(k) = Self::handle_key_event(&msg) {
                            if k.eq_ignore_ascii_case("s") {
                                break;
                            }
                        }
                    }
                    WM_KEYUP => {
                        // println!("Key released: {}", msg.wParam);
                        Self::handle_key_event(&msg);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_key_event(msg: &MSG) -> Option<String> {
        let vk_code = msg.wParam as u32; // Virtual key code
        let mut buffer = [0u16; 4]; // Buffer for Unicode characters
        let mut key_state = [0u8; 256]; // Keyboard state array

        unsafe {
            // Get the current keyboard state
            if GetKeyboardState(key_state.as_mut_ptr()) != 0 {
                // Translate the virtual key code into a Unicode character
                let chars_copied = ToUnicode(
                    vk_code,
                    (msg.lParam >> 16) as u32 & 0xFF, // Scan code from lParam
                    key_state.as_ptr(),
                    buffer.as_mut_ptr(),
                    buffer.len() as i32,
                    0,
                );

                if chars_copied > 0 {
                    // Convert the UTF-16 result to a Rust String
                    let result = String::from_utf16_lossy(&buffer[..chars_copied as usize]);
                    // println!("Key pressed: {}", result);
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub unsafe fn destroy(hwnd: &HWND) -> BOOL {
        DestroyWindow(*hwnd)
    }
}
