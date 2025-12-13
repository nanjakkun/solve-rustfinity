/*
You need to create an FSA that can recognize the pattern "ab*c", where:

    'a' is followed by zero or more **'b'**s and then followed by 'c'.

You will implement a function recognize_pattern that takes a string slice as input and returns a boolean indicating whether the input string matches the pattern.

    Implement the finite state automaton using Rust's enums and the match statement.
    The function should handle empty strings and any invalid input gracefully.
    Your FSA should consist of states and transitions implemented as Rust enums and functions.
    You must handle state transitions explicitly using the match statement.
*/

pub fn recognize_pattern(input: &str) -> bool {
    let mut state = 0; // 0: Start, 1: SeenA, 2: Accept, 3: Reject

    for c in input.chars() {
        state = match state {
            0 => {
                // Start
                if c == 'a' { 1 } else { 3 }
            }
            1 => match c {
                // SeenA
                'b' => 1,
                'c' => 2,
                _ => 3,
            },
            2 => 3, // Accept -> Reject (if input continues)
            _ => 3, // Reject
        };
    }

    state == 2
}
