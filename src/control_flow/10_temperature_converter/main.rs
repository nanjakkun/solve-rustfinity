/*
    The function should correctly convert temperatures between Celsius, Fahrenheit, and Kelvin.
    If the input unit or the desired output unit is not one of "C", "F", or "K", the function should return an error message: "Invalid unit".
    If the conversion is successful, the function should return the converted temperature as a float in the Ok variant of the Result.

    To convert Celsius to Fahrenheit: F = C * (9/5) + 32
    To convert Fahrenheit to Celsius: C = (F - 32) * (5/9)
    To convert Celsius to Kelvin: K = C + 273.15
    To convert Kelvin to Celsius: C = K - 273.15
*/
pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // TODO: Implement the function here
    match (from_unit, to_unit) {
        ("C", "F") => Ok(value * (9.0 / 5.0) + 32.0),
        ("F", "C") => Ok((value - 32.0) * (5.0 / 9.0)),
        ("C", "K") => Ok(value + 273.15),
        ("K", "C") => Ok(value - 273.15),
        ("F", "K") => Ok((value - 32.0) * (5.0 / 9.0) + 273.15),
        ("K", "F") => Ok((value - 273.15) * (9.0 / 5.0) + 32.0),
        _ => Err("Invalid unit".to_string()),
    }
}
