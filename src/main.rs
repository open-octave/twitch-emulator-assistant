extern crate user32;

use user32::{FindWindowA, SetForegroundWindow, ShowWindow};
use winapi::um::winuser::SW_RESTORE;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::ffi::OsString;
use std::iter::once;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{BOOL, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::ffi::CString;

use std::io;
use std::io::Write;

#[cfg(target_os = "macos")]
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

#[cfg(target_os = "macos")]
fn focus_window() {
    use std::process::Command;

    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"RetroArch\" to activate")
        .output()
        .expect("failed to execute process");
}

#[cfg(target_os = "macos")]
fn execute_command(command: &str) {
    focus_window();

    println!("Executing command: {}", command);

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match command {
        // Buttons
        "a" => enigo.key(Key::Unicode('x'), Click).unwrap(),
        "b" => enigo.key(Key::Unicode('z'), Click).unwrap(),
        "y" => enigo.key(Key::Unicode('a'), Click).unwrap(),
        "x" => enigo.key(Key::Unicode('s'), Click).unwrap(),

        // Directions
        "up" => enigo.key(Key::UpArrow, Click).unwrap(),
        "down" => enigo.key(Key::DownArrow, Click).unwrap(),
        "left" => enigo.key(Key::LeftArrow, Click).unwrap(),
        "right" => enigo.key(Key::RightArrow, Click).unwrap(),

        // Fallback
        _ => (),
    }

    println!("Command executed: {}", command);
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
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
            GetWindowTextW(window_handle, window_title.as_mut_ptr(), window_title.len() as i32);
            let title = OsString::from_wide(&window_title).to_string_lossy();
            println!("Current window: {}", title);
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn execute_command(command: &str) {
    match focus_window() {
        Ok(_) => println!("Focused on RetroArch window"),
        Err(e) => println!("{}", e),
    }

    println!("Executing command: {}", command);

    // Add implementation for Windows here

    println!("Command executed: {}", command);
}

#[tokio::main]
async fn main() {
    println!("Starting Twitch Game Emulator Assistant");

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login,
                        msg.sender.name,
                        msg.message_text.trim()
                    );

                    let raw_message = msg.message_text;
                    let sanitized_command = raw_message.to_lowercase().trim().to_string();

                    let permitted_commands =
                        vec!["a", "b", "x", "y", "up", "down", "left", "right"];

                    match sanitized_command {
                        _ if permitted_commands.contains(&sanitized_command.as_str()) => {
                            execute_command(&sanitized_command);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    });

    let mut channel = String::new();

    print!("Enter channel name: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut channel) {
        Ok(_) => {
            channel = channel.trim().to_string();
            println!("Joining channel: {}", channel);
            client.join(channel.to_owned()).unwrap();
            join_handle.await.unwrap();
        }
        Err(error) => println!("Error reading input: {}", error),
    }
}
