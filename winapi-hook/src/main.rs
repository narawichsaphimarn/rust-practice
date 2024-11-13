extern crate winapi;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{CallNextHookEx, SetWindowsHookExA, WH_MOUSE_LL, WM_LBUTTONDOWN,
                          WM_RBUTTONDOWN, UnhookWindowsHookEx, WM_LBUTTONUP, WM_RBUTTONUP,
                          WM_MBUTTONDOWN, SetWindowsHookExW};

fn main() {
    unsafe {
        let hook = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_hook_proc), ptr::null_mut(), 0);
        if hook.is_null() {
            println!("Failed to set hook.");
        } else {
            println!("Hook installed.");
        }

        // ลูปทำให้ hook ทำงาน
        loop {}
    }
}

unsafe extern "system" fn mouse_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 {
        match w_param as u32 {
            WM_LBUTTONDOWN => {
                println!("Left mouse button clicked.");
            },
            _ => {}
        }
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}

// pub fn block_event_click() -> HHOOK {
//     unsafe {
//         println!("mouse hook pressed");
//         let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), ptr::null_mut(), 0);
//         hook
//     }
// }
//
// pub fn unblock_hook(hook: HHOOK) {
//     unsafe {
//         UnhookWindowsHookEx(hook);
//     }
// }