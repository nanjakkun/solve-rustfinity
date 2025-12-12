/*
This challenge is about basic mathematical operations. You will be given 2 numbers a and b. You need to perform the following operations:

    Sum of a and b
    Difference of a and b
    Multiplication of a and b
    Division of a and b

You need to return a tuple containing the results of the above operations in the same order. (sum, difference, multiply, divide)
*/

pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    // TODO: Return a tuple of 4 values: (sum, difference, multiply, divide)
    return (a + b, a - b, a * b, a / b);
}
