use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::io;
use std::io::Write;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

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
