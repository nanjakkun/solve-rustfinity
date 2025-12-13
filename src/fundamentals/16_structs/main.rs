/*
Define a struct called Person with the fields name and age.
Implement the function is_adult to determine if a person is an adult (18 or older).
Make sure the struct and it's fields are public.
*/
pub struct Person {
    // Define fields here
    // Read the description
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    person.age >= 18
}
// Finish the function
