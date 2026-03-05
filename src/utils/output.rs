use anyhow::Result;
use serde::Serialize;

/// Print JSON to stdout (for Agent consumption)
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Print info message to stderr (for human consumption)
pub fn print_info(message: &str) {
    eprintln!("[INFO] {}", message);
}

/// Print error message to stderr (for human consumption)
pub fn print_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

/// Print warning message to stderr (for human consumption)
pub fn print_warning(message: &str) {
    eprintln!("[WARNING] {}", message);
}
