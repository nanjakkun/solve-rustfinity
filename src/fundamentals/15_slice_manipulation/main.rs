/*
Write a function update_slice(slice: &mut [i32], indices: &[usize], value: i32) that updates specific elements of a mutable slice. The function should:

    Take a mutable slice of integers (slice) as the first argument.
    Take a slice of indices (indices) that specify which elements of the mutable slice to update.
    Update each specified index in the slice to the given value.

The function should handle the following:

    If an index in indices is out of bounds for the slice, the function should skip it without causing a panic.
    Modify only the elements specified by valid indices.
    Ensure that out-of-bound indices in the indices slice do not cause runtime errors.
    Remember that slices are views into arrays or vectors; they cannot be resized, but their contents can be modified.
*/

pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here

    for idx in indices {
        if let Some(e) = slice.get_mut(*idx) {
            *e = value;
        }
    }
}
