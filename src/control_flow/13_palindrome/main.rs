/*
    The function should cover the numbers from start to end inclusive.
    It should check each non-negative number to determine if it is a palindrome.
    The function should return the numerically least palindrome found, or None if no palindromes exist in the range.
    You must handle "reversed" ranges, when start is greater than end.
*/
pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let (min, max) = if start < end {
        (start, end)
    } else {
        (end, start)
    };

    (min..=max).filter(|&n| n >= 0).find(|&n| {
        let s = n.to_string();
        s.chars().eq(s.chars().rev())
    })
}
