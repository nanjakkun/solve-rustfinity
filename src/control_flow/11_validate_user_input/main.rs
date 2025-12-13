/*
Your task is to implement a function validate_user(age: i32, email: &str) -> Result<(), String> that validates the user's age and email. The function should follow these rules:

    If the age is less than 0 or greater than 120, return an error with the message "Invalid age".
    If the email does not contain an '@' symbol, return an error with the message "Invalid email".
    If both the age and email are valid, return Ok(()).
*/
pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    // Implement here
    if age < 0 || age > 120 {
        return Err("Invalid age".to_string());
    }
    if !email.contains('@') {
        return Err("Invalid email".to_string());
    }
    Ok(())
}
