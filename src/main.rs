#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::iter::once;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{FindWindowW, SetForegroundWindow};

use enigo::{Enigo, KeyboardControllable};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

#[cfg(target_os = "windows")]
fn focus_window() {
    let window_name = OsStr::new("RetroArch 0.9.11 x64")
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    unsafe {
        let hwnd = FindWindowW(null_mut(), window_name.as_ptr());
        if hwnd != null_mut() {
            SetForegroundWindow(hwnd);
        }
    }
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
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );

                    let command = &msg.message_text.to_lowercase();
                    execute_command(command);
                }
                _ => {}
            }
        }
    });

    client.join("pcdsandwichman".to_owned()).unwrap();

    join_handle.await.unwrap();
}
