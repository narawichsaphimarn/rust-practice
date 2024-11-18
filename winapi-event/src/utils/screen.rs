use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub struct ScreenUtil;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
}

#[cfg(target_os = "windows")]
impl ScreenUtil {
    pub fn get_screen_metrics() -> Screen {
        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            let screen = Screen {
                width: screen_width,
                height: screen_height,
            };
            screen
        }
    }

    pub fn scale_coordinates(
        x: i32,
        y: i32,
        src_width: i32,
        src_height: i32,
        dest_width: i32,
        dest_height: i32,
    ) -> (i32, i32) {
        let scaled_x = (x * dest_width) / src_width;
        let scaled_y = (y * dest_height) / src_height;
        (scaled_x, scaled_y)
    }
}
