use std::io::Write;

pub fn ask_confirmation(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();

    return matches!(input.as_str(), "y" | "yes");
}
