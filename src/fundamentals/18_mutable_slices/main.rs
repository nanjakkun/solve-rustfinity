/*
Write a function modify_elements(slice: &mut [i32]) that modifies the elements of a mutable slice of integers in the following way:

    Double each even number in the slice.
    Subtract 1 from each odd number in the slice.
    Modify the elements directly without creating a new collection.
*/
pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for e in slice.iter_mut() {
        if *e % 2 == 0 {
            *e *= 2;
        } else {
            *e -= 1;
        }
    }
}
