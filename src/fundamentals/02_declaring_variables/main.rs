pub fn calculate_area() -> u32 {
    // TODO: Implement the function here
    // 1. Declare a variable named width
    let width = 1;
    // 2. Declare a variable named height
    let height = 2;
    // 3. Run the `prints_values` function with the width and height variables
    prints_values(width, height);
    // 4. Return the multiplication of width and height
    return width * height;
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}
