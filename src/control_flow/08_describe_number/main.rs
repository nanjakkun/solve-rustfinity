/*
    If the number is positive and even, return "Positive even".
    If the number is positive and odd, return "Positive odd".
    If the number is negative and even, return "Negative even".
    If the number is negative and odd, return "Negative odd".
    If the number is zero, return "Zero".
*/
pub fn describe_number(n: i32) -> String {
    if n == 0 {
        "Zero".to_string()
    } else if n > 0 {
        if n % 2 == 0 {
            "Positive even".to_string()
        } else {
            "Positive odd".to_string()
        }
    } else {
        if n % 2 == 0 {
            "Negative even".to_string()
        } else {
            "Negative odd".to_string()
        }
    }
}
