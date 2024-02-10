use rand::{thread_rng, Rng};
use std::char::from_u32;
use std::process::Command;

fn main() {
    let mut password: String = String::new();

    for _ in 0..15 {
        let random_number: u32 = thread_rng().gen_range(33..126);
        let char: char = from_u32(random_number).unwrap();
        password.push(char);
    }
    let command = format!(
        "printf '%s' '{}' | {} -selection clipboard",
        password, "xclip"
    );
    Command::new("sh")
        .args(["-c", command.as_str()])
        .spawn()
        .expect("Failed to copy password");
    Command::new("sh")
        .args([
            "-c",
            "notify-send 'password-generator' 'Password has been copied to your clipboard'",
        ])
        .spawn()
        .expect("Failed to send desktop notification");
}
