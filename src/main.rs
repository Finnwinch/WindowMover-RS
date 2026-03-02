#![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VK_A,
    VK_D,
    VK_RSHIFT,
    VK_S,
    VK_W
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow,
    GetWindowRect,
    SetWindowPos,
    SWP_NOSIZE,
    SWP_NOZORDER
};

fn main() {
    let step = 10;

    loop {
        unsafe {
            if (GetAsyncKeyState(VK_RSHIFT.0 as i32) as u16 & 0x8000) != 0 {
                
                let hwnd = GetForegroundWindow();
                
                if !hwnd.is_invalid() {
                    let mut rect = RECT::default();
                    
                    if GetWindowRect(hwnd, &mut rect).is_ok() {
                        let mut dx = 0;
                        let mut dy = 0;

                        if is_pressed(VK_W.0 as i32) { dy -= step; }
                        if is_pressed(VK_S.0 as i32) { dy += step; }
                        if is_pressed(VK_A.0 as i32) { dx -= step; }
                        if is_pressed(VK_D.0 as i32) { dx += step; }

                        if dx != 0 || dy != 0 {
                            move_window(hwnd, rect, dx, dy);
                            sleep(Duration::from_millis(10));
                        }
                    }
                }
            }
        }
        sleep(Duration::from_millis(10));
    }
}

unsafe fn is_pressed(vkey: i32) -> bool {
    (GetAsyncKeyState(vkey) as u16 & 0x8000) != 0
}

unsafe fn move_window(hwnd: HWND, rect: RECT, dx: i32, dy: i32) {
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    let _ = SetWindowPos(
        hwnd,
        None, 
        rect.left + dx,
        rect.top + dy,
        width,
        height,
        SWP_NOSIZE | SWP_NOZORDER,
    );
}