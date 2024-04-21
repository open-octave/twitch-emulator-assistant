use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::io;
use std::io::Write;

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
