// Define a struct named `Logger`
// Implement an associated function `log_message`
// That accepts a `&str` and prints the output.

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}

pub struct Logger {}

impl Logger {
    pub fn log_message(message: &str) -> () {
        println!("{}", message);
    }
}
