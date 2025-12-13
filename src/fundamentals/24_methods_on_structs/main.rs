/*
    Define a struct Counter with a single field count of type i32.

    Define a new associated function that acts as a constructor and initializes the count field to 0.

    Implement the following methods for Counter:
        increment: Increments the counter by 1.
        decrement: Decrements the counter by 1.
        get_count: Returns the current count.

    Ensure these methods use the correct self parameter type (&self or &mut self) based on their behavior.

    Make the count field private and provide a public constructor Counter::new that initializes the count to 0.
*/

// 1. Define the struct
pub struct Counter {
    count: i32,
}

// 2. Implement the associated function and methods
impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) -> () {
        self.count += 1;
    }

    pub fn decrement(&mut self) -> () {
        self.count -= 1;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }
}

// Example use case
pub fn main() {
    let mut counter = Counter::new();

    counter.increment();
    assert_eq!(counter.get_count(), 1);

    counter.increment();
    counter.increment();
    assert_eq!(counter.get_count(), 3);

    counter.decrement();
    assert_eq!(counter.get_count(), 2);

    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get_count(), 0);
}
