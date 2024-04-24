use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

fn focus_window() {
    use std::process::Command;

    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"RetroArch\" to activate")
        .output()
        .expect("failed to execute process");
}

pub fn execute_command(command: &str) {
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
