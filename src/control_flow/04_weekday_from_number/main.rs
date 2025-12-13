/*
Your task is to implement a function, weekday_from_number(day: u8) -> &'static str, that takes a number as input and returns the corresponding weekday as a string. If the input number is not within the range of 1 to 7, the function should return "Invalid day number".
Requirements

    If the input number is 1, the function should return "Monday".
    If the input number is 2, the function should return "Tuesday".
    If the input number is 3, the function should return "Wednesday".
    If the input number is 4, the function should return "Thursday".
    If the input number is 5, the function should return "Friday".
    If the input number is 6, the function should return "Saturday".
    If the input number is 7, the function should return "Sunday".
    If the input number is not between 1 and 7, the function should return "Invalid day number".
*/

pub fn weekday_from_number(day: u8) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day number",
    }
}
