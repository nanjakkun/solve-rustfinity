/*
The append_suffix function should take a mutable reference to the input String and append the given suffix to it.
*/

pub fn append_suffix(s: &mut String, suffix: &str) {
    // Your code here...
    return s.push_str(suffix);
}
