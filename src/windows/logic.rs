use std::os::windows::ffi::OsStringExt;

use user32::{FindWindowA, SetForegroundWindow, ShowWindow};
use winapi::um::winuser::SW_RESTORE;

use winapi::shared::minwindef::{BOOL, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW};
extern crate user32;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let length = GetWindowTextLengthW(hwnd) as usize;
    let mut buffer = vec![0u16; length + 1];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
    let title = OsString::from_wide(&buffer).to_string_lossy().into_owned();

    if title.to_lowercase().contains("retroarch") {
        // Convert the LPARAM to a mutable reference to a String
        let window_title = &mut *(lparam as *mut String);
        *window_title = title;
        return false as BOOL; // Stop enumeration
    }

    true as BOOL // Continue enumeration
}

fn find_retroarch_window() -> Option<String> {
    let mut window_title: String = String::new();
    unsafe {
        EnumWindows(
            Some(enum_windows_callback),
            &mut window_title as *mut _ as LPARAM,
        );
    }

    if window_title.is_empty() {
        None
    } else {
        Some(window_title)
    }
}

fn focus_window() -> Result<(), String> {
    let window_name = match find_retroarch_window() {
        Some(name) => name,
        None => return Err("No RetroArch window found.".to_string()),
    };

    let window_handle =
        unsafe { user32::FindWindowA(null_mut(), window_name.as_ptr() as *const i8) };

    if window_handle.is_null() {
        Err("No RetroArch window found.".to_string())
    } else {
        unsafe {
            SetForegroundWindow(window_handle);
            // ShowWindow(window_handle, SW_RESTORE);

            // Log the current forground window
            let mut window_title = vec![0u16; 100];
            GetWindowTextW(
                window_handle,
                window_title.as_mut_ptr(),
                window_title.len() as i32,
            );
            let title = OsString::from_wide(&window_title).to_string_lossy();
            println!("Current window: {}", title);
        }
        Ok(())
    }
}

fn execute_command(command: &str) {
    match focus_window() {
        Ok(_) => println!("Focused on RetroArch window"),
        Err(e) => println!("{}", e),
    }

    println!("Executing command: {}", command);

    // Add implementation for Windows here

    println!("Command executed: {}", command);
}
