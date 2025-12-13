use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    // TODO: Implement logic here to return the median of the list
    numbers.sort();
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2] + numbers[len / 2 - 1]) as f32 / 2.0
    } else {
        numbers[len / 2] as f32
    }
}

/*
   Sort the list of numbers.
   If the number of elements is odd, return the middle element.
   If the number of elements is even, return the average of the two middle elements.
*/
pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    // TODO: Implement logic here to return the mode of the list
    let mut map = HashMap::new();
    for &num in numbers {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut modes = Vec::new();
    for (&num, &count) in map.iter() {
        if count > max_count {
            max_count = count;
            modes.clear();
            modes.push(num);
        } else if count == max_count {
            modes.push(num);
        }
    }
    modes
}
