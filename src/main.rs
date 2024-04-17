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
#[cfg(target_os = "windows")]
use winput::{Button, Vk};

use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::io;
use std::io::Write;

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
#[cfg(target_os = "macos")]
fn execute_command(command: &str) {
    println!("Running command: {}", command);
    focus_window();

    println!("Executing command");

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match command {
        "a" => enigo.key(Key::Unicode('z'), Click).unwrap(),
        "b" => enigo.key(Key::Unicode('x'), Click).unwrap(),
        "y" => enigo.key(Key::Unicode('c'), Click).unwrap(),
        "x" => enigo.key(Key::Unicode('v'), Click).unwrap(),
        "up" => enigo.key(Key::Unicode('w'), Click).unwrap(),
        "down" => enigo.key(Key::Unicode('s'), Click).unwrap(),
        "left" => enigo.key(Key::Unicode('a'), Click).unwrap(),
        "right" => enigo.key(Key::Unicode('d'), Click).unwrap(),
        _ => (),
    }

    println!("Command executed");
}

#[cfg(target_os = "windows")]
fn execute_command(command: &str) {
    println!("Running command: {}", command);
    println!("Executing command");

    match command {
        "a" => winput::press(Vk::Z).unwrap(),
        "b" => winput::press(Vk::X).unwrap(),
        "y" => winput::press(Vk::C).unwrap(),
        "x" => winput::press(Vk::V).unwrap(),
        "up" => winput::press(Vk::W).unwrap(),
        "down" => winput::press(Vk::S).unwrap(),
        "left" => winput::press(Vk::A).unwrap(),
        "right" => winput::press(Vk::D).unwrap(),
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
