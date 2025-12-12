/*
Implement the function get_first_element:

    Takes two parameters:
        A vector of integers (Vec<i32>).
        A minimum allowed value (i32).
    Use the .first() method to retrieve the first element of the vector, this returns Option<&T>.
    If the value is None, convert it to a Result<T, E> using the .ok_or() method with the error message "Vector is empty".
    Then run a validation check to ensure the first element is greater than or equal to the minimum allowed value provided. If not, return an error with message "First element is below the minimum allowed value".
    If everything is ok, return a Ok(T) with the first element.
*/
pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    numbers
        .first()
        .ok_or("Vector is empty".to_string())
        .and_then(|&x| {
            if x >= min_value {
                Ok(x)
            } else {
                Err("First element is below the minimum allowed value".to_string())
            }
        })
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
