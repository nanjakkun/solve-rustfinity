/*
You need to implement the function create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String) that takes an integer i32, a float f64, and a string slice &str as input and returns them as a tuple. The string slice should be converted into a String type.

    The create_tuple function should return a tuple containing the three input values in order.
    The string slice input should be converted into a String before returning.
*/
pub fn create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String) {
    // TODO: Implement the function here

    (a, b, c.to_string())
}
