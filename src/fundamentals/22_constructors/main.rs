/*
In this challenge, you will implement a constructor for a struct that represents a Book. A Book will have the following fields:

    title: A string that represents the book's title.
    author: A string that represents the book's author.
    year: An integer that represents the year the book was published.
    likes: An integer that represents the number of likes the book has received. Default value is 0.

The constructor function, Book::new, should take three parameters (title, author, and year) and return a fully initialized Book.
Requirements

    Implement a constructor function Book::new that:
        Takes three arguments: title: &str, author: &str, and year (integer).
        Returns a Book instance with the specified values and default likes value of 0.

    Remember to use pub for fields (required for testing).
*/

pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    // 2. Define the `new` associated function
    pub fn new(title: &str, author: &str, year: i32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}
