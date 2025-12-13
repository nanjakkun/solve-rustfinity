/*
Implement a function print_message that accepts any type that implements the AsRef<str> trait. This function should:

    Accept a single argument of a generic type that implements AsRef<str>.
    Borrow the input as a string slice (&str) and print it to the console using println!.
*/
pub fn print_message<T: AsRef<str>>(message: T) {
    println!("{}", message.as_ref());
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
