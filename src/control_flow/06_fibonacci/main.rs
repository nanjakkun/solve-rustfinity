use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static MEMO: OnceLock<Mutex<HashMap<u32, u32>>> = OnceLock::new();

pub fn fibonacci(n: u32) -> u32 {
    let cache = MEMO.get_or_init(|| Mutex::new(HashMap::new()));

    // Check cache
    {
        let guard = cache.lock().unwrap();
        if let Some(&val) = guard.get(&n) {
            return val;
        }
    }

    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    };

    // Save to cache
    {
        let mut guard = cache.lock().unwrap();
        guard.insert(n, result);
    }

    result
}
