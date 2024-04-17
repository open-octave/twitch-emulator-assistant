use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

pub fn focus_window() {
    use std::process::Command;

    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"RetroArch\" to activate")
        .output()
        .expect("failed to execute process");
}

pub fn execute_command(command: &str) {
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
