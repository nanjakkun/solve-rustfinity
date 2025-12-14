/*
    Define an enum OrderStatus with the following variants:
        Pending — a unit variant representing an order that is not yet processed.
        Shipped — a unit variant representing an order that has been shipped.
        Cancelled(String) — a tuple variant with a reason for cancellation.

    Use the #[derive(PartialEq)] attribute to derive the PartialEq trait for the enum.

    Write tests to ensure that the derived implementation works as expected.
*/

// Finish the enum definition
#[derive(PartialEq, Debug)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}

// Example use case
pub fn main() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Pending;
    assert_eq!(status1, status2);

    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());
    assert_eq!(cancelled1, cancelled2);

    let cancelled3 = OrderStatus::Cancelled("Customer request".to_string());
    assert_ne!(cancelled1, cancelled3);
}
