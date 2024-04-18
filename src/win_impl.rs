use std::{ffi::OsStr, iter::once, ptr::null_mut};

use winapi::um::winuser::{FindWindowW, SendInput, SetForegroundWindow, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP};

// pub fn focus_window() {
//     let window_name = OsStr::new("RetroArch 0.9.11 x64")
//         .encode_wide()
//         .chain(once(0))
//         .collect::<Vec<u16>>();

//     unsafe {
//         let hwnd = FindWindowW(null_mut(), window_name.as_ptr());
//         if hwnd != null_mut() {
//             SetForegroundWindow(hwnd);
//         }
//     }
// }

fn press_key(command: &str) {
    let vk_code = match command {
        "a" => 0x5A, // 'Z' key
        "b" => 0x58, // 'X' key
        "x" => 0x43, // 'C' key
        "y" => 0x56, // 'V' key
        "up" => 0x57, // 'W' key
        "down" => 0x53, // 'S' key
        "left" => 0x41, // 'A' key
        "right" => 0x44, // 'D' key
        _ => return, // Do nothing if the command is unrecognized
    };

    let mut inputs = [INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { std::mem::zeroed() },
    }; 2];

    unsafe {
        // Key down event
        inputs[0].u.ki_mut().wVk = vk_code as u16;
        inputs[0].u.ki_mut().dwFlags = 0; // Key press
    
        // Key up event
        inputs[1].u.ki_mut().wVk = vk_code as u16;
        inputs[1].u.ki_mut().dwFlags = KEYEVENTF_KEYUP; // Key release
    }

    // Call SendInput with array of INPUT structures-
    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(), // Corrected to mutable pointer
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

pub fn execute_command(command: &str) {
    println!("Running command: {}", command);
    // focus_window();

    println!("Executing command");
    press_key(command);
    println!("Command executed");
}
