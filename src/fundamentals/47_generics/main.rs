/*
    Define a generic struct ItemContainer<T> with a single field item: T.
    Implement a method get_item that returns a reference to the item.
*/

pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    // TODO: Implement the `get_item` method to return a reference to the item.
    pub fn get_item(&self) -> &T {
        &self.item
    }
}

// Example usage
pub fn main() {
    let item_1 = ItemContainer { item: 42 };
    assert_eq!(*item_1.get_item(), 42);

    let item_2 = ItemContainer {
        item: String::from("Hello"),
    };

    assert_eq!(item_2.get_item(), "Hello");
}
