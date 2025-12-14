/*
    Filter duplicates: Ensure that the resulting collection contains only unique items.
    Ignore invalid entries: Exclude entries that are empty or only consist of whitespace.
    Sort the output: Return the items sorted in ascending lexicographical order.
    Flexibility: The function should work with any iterator that has String, &String, or &str items.
*/

// 1. Finish the function
pub fn unique_items<I, T>(iter: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut items: Vec<String> = iter
        .map(|s| s.as_ref().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    items.sort();
    items.dedup();
    items
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
