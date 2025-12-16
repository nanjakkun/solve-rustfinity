/*
You need to implement a method check_validity(&self) for the BookItem enum that returns a bool indicating whether the item is valid based on the following rules:

    Book:
        The number of pages must be greater than 0.
        If there is a discount:
            Must be non-negative (≥ 0)
            Must not exceed 50% (≤ 50)

    EBook:
        The title (a String) must be non-empty.
        The second field in the tuple must be greater than 0.

    Collection:
        Must not be empty
        ALL items in the collection must be valid
        This means a collection containing even one invalid item is considered invalid

    OutOfPrint:
        This variant is always considered invalid.
*/
#[derive(Debug, Clone)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    pub fn check_validity(&self) -> bool {
        match self {
            BookItem::Book { pages, discount } => {
                *pages > 0 && discount.map_or(true, |d| d >= 0 && d <= 50)
            }
            BookItem::EBook(title, (_, v)) => !title.is_empty() && *v > 0,
            BookItem::Collection(items) => {
                !items.is_empty() && items.iter().all(|item| item.check_validity())
            }
            BookItem::OutOfPrint => false,
        }
    }
}

// Example usage
pub fn main() {
    let book_a = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
    let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

    assert!(
        !book_a.check_validity(),
        "Book with discount > 50 should be invalid"
    );
    assert!(
        ebook_b.check_validity(),
        "EBook with valid title and tuple should be valid"
    );
    assert!(
        !collection_c.check_validity(),
        "Collection containing invalid items should be invalid"
    );
    assert!(
        !BookItem::OutOfPrint.check_validity(),
        "OutOfPrint should always be invalid"
    );
}
