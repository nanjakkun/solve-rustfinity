/*
Create an enum Card with the following variants:

    King, Queen, Jack (unit variants for face cards).
    Numbered(u8, String) representing a numbered card with its value and suit.

Write a function card_description that takes a Card and returns a description of the card:

    For King, return "King".
    For Queen, return "Queen".
    For Jack, return "Jack".
    For Numbered(value, suit), return "{value} of {suit}", e.g., "7 of Hearts".
    Ignore error handling for this challenge.
*/
pub enum Card {
    // Define the Card variants here
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    // Your code here...
    match card {
        Card::King => "King".to_string(),
        Card::Queen => "Queen".to_string(),
        Card::Jack => "Jack".to_string(),
        Card::Numbered(value, suit) => format!("{} of {}", value, suit),
    }
}
