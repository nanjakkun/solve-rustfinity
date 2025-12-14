/*
In this challenge, you will implement a struct named MutableTextFinder that holds a mutable reference to a String. This struct will allow for both searching and modifying the content of the String.

The MutableTextFinder struct should provide the following functionality:

    new: Creates a new instance of MutableTextFinder with the given content.
    find_first: Searches for the first line containing a given keyword and returns it as an immutable reference (Option<&str>).
    replace_lines: Replaces all lines containing a given keyword with a replacement string.
    get_text: Returns the reference to the content.
    Searches should be case-sensitive.
*/

// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    text: &'a mut String,
}

// 2. Implement the methods for the struct
impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }
    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(keyword))
    }
    pub fn replace_lines(&mut self, keyword: &str, replacement: &str) {
        *self.text = self
            .text
            .lines()
            .map(|line| {
                if line.contains(keyword) {
                    replacement
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
