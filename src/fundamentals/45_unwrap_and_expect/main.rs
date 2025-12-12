/*
    read_file_to_string: This function takes a file path as input, attempts to read its contents, and returns the contents as a String. Use expect to handle any file I/O errors with a custom error message of exactly Failed to read file: <path>.
    get_env_variable: This function retrieves the value of an environment variable by name. Use unwrap to panic if the variable is not set.
*/
pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    std::fs::read_to_string(path).expect(&format!("Failed to read file: {}", path))
}

pub fn get_env_variable(key: &str) -> String {
    // 2. Implement the function
    std::env::var(key).unwrap()
}

/// Example usage
pub fn main() {
    // Example 1: Using read_file_to_string
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    // Example 2: Using get_env_variable
    std::env::set_var("EXAMPLE_KEY", "example_value");
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    // Must panic
    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}
