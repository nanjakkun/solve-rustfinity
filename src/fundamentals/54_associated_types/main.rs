/*

Define a trait KeyValueStore with an associated type Key and Value. Implement this trait for the struct InMemoryStore. The implementation should allow storing and retrieving key-value pairs of the specified types.

Define a trait KeyValueStore with:
    An associated type Key.
    An associated type Value.
    Methods:
        set to add a key-value pair.
        get takes a reference of &Key and returns Option<&Value>.

    Create a struct InMemoryStore that uses a HashMap to store key-value pairs. Implement the KeyValueStore trait for this struct.

    Make sure all relevant values are public so that they can be accessed from outside the module (essential to pass the tests).

*/

use std::collections::HashMap;

// 1. Finish the trait definition
pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

// 2. Implement the trait for InMemoryStore
// Make sure the fields are public
pub struct InMemoryStore<Key, Value> {
    pub storage: HashMap<Key, Value>,
}

impl<Key, Value> KeyValueStore for InMemoryStore<Key, Value>
where
    Key: Eq + std::hash::Hash,
{
    type Key = Key;
    type Value = Value;

    fn set(&mut self, key: Self::Key, value: Self::Value) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.storage.get(key)
    }
}

// Example usage
pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));

    store.set("language".to_string(), "Rust".to_string());
    assert_eq!(
        store.get(&"language".to_string()),
        Some(&"Rust".to_string())
    );

    assert_eq!(store.get(&"non_existent".to_string()), None);
}
