// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() -> (
    impl Fn(f32, f32) -> f32,
    impl FnMut(&mut f32, f32) -> (),
    impl FnOnce(String) -> String,
) {
    // 2. Implement calculate_total closure here
    let calculate_total = |amount, rate| amount + amount * rate;

    // 3. Implement apply_discount closure here
    let apply_discount = |amount: &mut f32, discount: f32| *amount -= discount;

    // 4. Implement checkout_cart closure here
    let checkout_cart = |cart_details| format!("Checkout complete: {}", &cart_details);

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
