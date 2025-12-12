/*
In this challenge, you're given a function, parse_percentage that takes a string as input and returns a Result type. The function should parse the input string as a percentage and return the percentage as a u8 if the input is valid. If the input is invalid, the function should return an error message as a String.

Parsing from a string to a number can fail for many reasons. For example, the input string may not be a valid number, or it may be a valid number but not a valid percentage. Your task is to handle these errors gracefully and return an error message that explains what went wrong.

Complete the function, if the parsing was successful return a success variant of the Result, if there was an error in parsing, return an error variant of the Result with an error message.
*/

pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    match input.parse::<u8>() {
        Ok(v) => {
            if v > 100 {
                Err("Percentage out of range".to_string())
            } else {
                Ok(v)
            }
        }
        Err(_) => Err("Invalid input".to_string()),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
