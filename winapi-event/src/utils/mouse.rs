use serde::{Deserialize, Serialize};
use std::fmt;
#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::{POINT, RECT},
    um::winuser::{ClipCursor, GetCursorPos, SetCursorPos, ShowCursor},
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Mouse {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum PositionAtEdge {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

pub fn map_from_string(edge: String) -> PositionAtEdge {
    if PositionAtEdge::Bottom
        .to_string()
        .eq_ignore_ascii_case(&edge)
    {
        PositionAtEdge::Bottom
    } else if PositionAtEdge::Left.to_string().eq_ignore_ascii_case(&edge) {
        PositionAtEdge::Left
    } else if PositionAtEdge::Right
        .to_string()
        .eq_ignore_ascii_case(&edge)
    {
        PositionAtEdge::Right
    } else if PositionAtEdge::Top.to_string().eq_ignore_ascii_case(&edge) {
        PositionAtEdge::Top
    } else {
        PositionAtEdge::None
    }
}

impl fmt::Display for PositionAtEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PositionAtEdge::Bottom => write!(f, "bottom"),
            PositionAtEdge::Top => write!(f, "top"),
            PositionAtEdge::Left => write!(f, "left"),
            PositionAtEdge::Right => write!(f, "right"),
            PositionAtEdge::None => write!(f, "none"),
        }
    }
}

pub struct MouseUtil;

#[cfg(target_os = "windows")]
impl MouseUtil {
    pub fn get_cursor_point() -> Mouse {
        let mut cursor_pos = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut cursor_pos);
        }
        Mouse {
            x: cursor_pos.x as f64,
            y: cursor_pos.y as f64,
        }
    }

    pub fn lock_cursor(cursor_pos: Mouse) {
        unsafe {
            let rect = RECT {
                left: cursor_pos.x as i32,
                top: cursor_pos.y as i32,
                right: (cursor_pos.x + 1.0) as i32,
                bottom: (cursor_pos.y + 1.0) as i32,
            };
            ClipCursor(&rect);
        }
    }

    pub fn unlock_cursor() {
        unsafe {
            ClipCursor(std::ptr::null());
        }
    }

    pub fn hidden_cursor() {
        unsafe { while ShowCursor(0) >= 0 {} }
    }

    pub fn show_cursor() {
        unsafe {
            ShowCursor(1);
        }
    }

    pub fn move_cursor(x: i32, y: i32) {
        loop {
            let success = unsafe { SetCursorPos(x, y) != 0 };
            if success {
                break;
            }
        }
    }
}

impl MouseUtil {
    pub fn check_position_at_edge(cursor_pos: Mouse, screen: Screen) -> Option<PositionAtEdge> {
        if cursor_pos.x <= 0.0 {
            Some(PositionAtEdge::Left)
        } else if cursor_pos.x >= (screen.width as f64) - 1.0 {
            Some(PositionAtEdge::Right)
        } else if cursor_pos.y <= 0.0 {
            Some(PositionAtEdge::Top)
        } else if cursor_pos.y >= (screen.height as f64) - 1.0 {
            Some(PositionAtEdge::Bottom)
        } else {
            Some(PositionAtEdge::None)
        }
    }

    pub fn revere_mouse_position(edge: PositionAtEdge, screen: Screen, cursor_pos: Mouse) {
        match edge {
            PositionAtEdge::Top => Self::move_cursor(
                cursor_pos.x as i32,
                screen.height - (cursor_pos.y as i32) - 5,
            ),
            PositionAtEdge::Bottom => Self::move_cursor(
                cursor_pos.x as i32,
                (cursor_pos.y as i32) - screen.height + 5,
            ),
            PositionAtEdge::Left => Self::move_cursor(
                screen.width - (cursor_pos.x as i32) - 5,
                cursor_pos.y as i32,
            ),
            PositionAtEdge::Right => Self::move_cursor(
                screen.width - (cursor_pos.x as i32) + 5,
                cursor_pos.y as i32,
            ),
            PositionAtEdge::None => (),
        }
    }

    pub fn get_revere_mouse_position(
        edge: PositionAtEdge,
        screen: Screen,
        cursor_pos: Mouse,
    ) -> Mouse {
        match edge {
            PositionAtEdge::Top => Mouse {
                x: cursor_pos.x,
                y: (screen.height - (cursor_pos.y as i32) - 5) as f64,
            },
            PositionAtEdge::Bottom => Mouse {
                x: cursor_pos.x,
                y: ((cursor_pos.y as i32) - screen.height + 5) as f64,
            },
            PositionAtEdge::Left => Mouse {
                x: (screen.width - (cursor_pos.x as i32) - 5) as f64,
                y: cursor_pos.y,
            },
            PositionAtEdge::Right => Mouse {
                x: (screen.width - (cursor_pos.x as i32) + 5) as f64,
                y: cursor_pos.y,
            },
            PositionAtEdge::None => Mouse { x: 0.0, y: 0.0 },
        }
    }

    pub fn mouse_different_pointer(
        current_point: &Mouse,
        source_screen: Screen,
        target_screen: Screen,
    ) -> Mouse {
        Mouse {
            x: (current_point.x * (source_screen.width as f64)) / (target_screen.width as f64),
            y: (current_point.y * (source_screen.height as f64)) / (target_screen.height as f64),
        }
    }
}
