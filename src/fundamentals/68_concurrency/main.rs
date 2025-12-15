/*
    The function should accept a Vec<T> and a value of type T.
    T can be, integer, floating-point, or any other number type in Rust.
    The function should return a vector of JoinHandle<T>.
*/

use std::thread;

pub fn concurrent_add<T>(items: Vec<T>, num: T) -> Vec<thread::JoinHandle<T>>
where
    T: std::ops::Add<Output = T> + Send + Copy + 'static,
{
    items
        .into_iter()
        .map(|item| thread::spawn(move || item + num))
        .collect()
}

// Example Usage
pub fn main() {
    {
        let mut list = vec![1, 2, 3, 4, 5];

        let handles = concurrent_add(list.clone(), 3);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 3);
        }
    }

    {
        let mut list = vec![10., 20., 30., 40., 50.];

        let handles = concurrent_add(list.clone(), 5.);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 5.);
        }
    }
}
