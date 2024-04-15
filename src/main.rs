#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::BOOL;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::LPARAM;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW, SetForegroundWindow};
#[cfg(target_os = "windows")]
use winput::Vk;

#[cfg(target_os = "macos")]
use enigo::{Enigo, KeyboardControllable};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::io;
use std::io::Write;

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut buffer = vec![0; GetWindowTextLengthW(hwnd) as usize + 1];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
    let title = OsString::from_wide(&buffer[..buffer.len() - 1]);

    if title.to_string_lossy().contains("RetroArch") {
        *(lparam as *mut HWND) = hwnd;
        0 // return false to stop enumerating
    } else {
        1 // continue enumerating
    }
}

#[cfg(target_os = "windows")]
fn focus_window() {
    let mut hwnd = null_mut();
    unsafe {
        EnumWindows(Some(enum_windows_proc), &mut hwnd as *mut _ as LPARAM);
        if hwnd != null_mut() {
            SetForegroundWindow(hwnd);
        }
    }
}

#[cfg(target_os = "windows")]
fn execute_command(command: &str) {
    println!("Running command: {}", command);
    focus_window();

    println!("Executing command");

    match command {
        "a" => winput::send(Vk::X),
        "b" => winput::send(Vk::Z),
        "y" => winput::send(Vk::A),
        "x" => winput::send(Vk::S),
        "up" => winput::send(Vk::UpArrow),
        "down" => winput::send(Vk::DownArrow),
        "left" => winput::send(Vk::LeftArrow),
        "right" => winput::send(Vk::RightArrow),
        _ => (),
    }

    println!("Command executed");
}

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
    println!("Running command: {}", command);
    focus_window();

    println!("Executing command");

    let mut enigo = Enigo::new();
    match command {
        "a" => enigo.key_click(enigo::Key::Layout('x')),
        "b" => enigo.key_click(enigo::Key::Layout('z')),
        "y" => enigo.key_click(enigo::Key::Layout('a')),
        "x" => enigo.key_click(enigo::Key::Layout('s')),
        "up" => enigo.key_click(enigo::Key::UpArrow),
        "down" => enigo.key_click(enigo::Key::DownArrow),
        "left" => enigo.key_click(enigo::Key::LeftArrow),
        "right" => enigo.key_click(enigo::Key::RightArrow),
        _ => (),
    }

    println!("Command executed");
}

#[tokio::main]
pub async fn main() {
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

                    let command = &msg.message_text.to_lowercase();

                    // If the command is a valid command, execute it
                    match command.as_str() {
                        "a" | "b" | "x" | "y" | "up" | "down" | "left" | "right" => {
                            execute_command(command.trim());
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
