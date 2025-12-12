/*
The logic of the function remains the same as the previous challenge, but the returned error type is what you need to change.

    Define an enum ParsePercentageError with the following variants:
        InvalidInput: for inputs that cannot be parsed as numbers.
        OutOfRange: for numbers that are not in the range 0-100.

    Implement the Error trait for ParsePercentageError. Use the std::error::Error trait and provide human-readable descriptions for each error.

    Update the parse_percentage function to:
        Return Ok(u8) if the input is a valid percentage (between 0 and 100).
        Return Err(ParsePercentageError::OutOfRange) if the number is out of range.
        Return Err(ParsePercentageError::InvalidInput) if the input is not a valid number.
*/

use std::error::Error;
use std::fmt;

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

// 2. Implement the `Error` trait
impl Error for ParsePercentageError {}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Invalid Input"),
            ParsePercentageError::OutOfRange => write!(f, "Number Out of Range"),
        }
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    match input.parse::<u8>() {
        Ok(v) => {
            if v > 100 {
                Err(ParsePercentageError::OutOfRange)
            } else {
                Ok(v)
            }
        }
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
