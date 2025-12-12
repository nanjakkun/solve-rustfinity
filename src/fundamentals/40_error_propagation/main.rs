/*
Your task is to implement a function that reads integers from a file, computes their sum, and gracefully propagates any errors using the ? operator.

Implement the function sum_integers_from_file:

    Takes the file path as a parameter.
    Reads the file line by line, assuming each line contains a single integer or invalid data.
    Computes and returns the sum of all integers as a Result<i32, io::Error>.
    Handles the following:
        If the file cannot be opened, propagate the io::Error.
        If a line cannot be parsed as an integer, propagate a custom io::Error with a meaningful message.

Handle errors cleanly and propagate them using the ? operator.
For invalid lines, create an io::Error with io::ErrorKind::InvalidData.
*/

use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let num = line?
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        sum += num;
    }
    Ok(sum)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
