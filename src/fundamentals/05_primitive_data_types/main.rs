/*
    Define a variable with type u8 and value 42
    Define variable with type f64 and value 3.14
    Define variable with type bool and value false
    Define variable with type char and value a
    In the end, return the tuple (u8, f64, bool, char) with the variables you defined.
*/

pub fn data_types() -> (u8, f64, bool, char) {
    // 1. Define variable of type `u8` and value `42`
    let v1: u8 = 42;

    // 2. Define variable of type `f64` and value `3.14`
    let v2: f64 = 3.14;

    // 3. Define variable of type `bool` and value `false`
    let v3: bool = false;

    // 4. Define variable of type `char` and value `a`
    let v4: char = 'a';

    // 5. Return a tuple with the variables in the order they were defined

    return (v1, v2, v3, v4);
}
