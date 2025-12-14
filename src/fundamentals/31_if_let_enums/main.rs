/*
You are provided an enum called Message with the following variants:

    Text(String): Represents a textual message.
    Number(i32): Represents a numerical message.
    Quit: Represents a command to quit.
    None: Represents no message.

Your task is to implement the function process_text_message that takes a reference to a Message enum. For the Text variant, the function should return "Processed Text: <content>", replacing <content> with the actual string.

If the input is any other variant of the enum, the function should return "Unhandled Message".

Try to solve this using the if let construct.
*/

pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_text_message(message: &Message) -> String {
    // Your code here...
    if let Message::Text(content) = message {
        format!("Processed Text: {content}")
    } else {
        String::from("Unhandled Message")
    }
}

pub fn main() {
    assert_eq!(
        process_text_message(&Message::Text(String::from("Hello"))),
        "Processed Text: Hello"
    );
    assert_eq!(
        process_text_message(&Message::Number(42)),
        "Unhandled Message"
    );
    assert_eq!(process_text_message(&Message::Quit), "Unhandled Message");
    assert_eq!(process_text_message(&Message::None), "Unhandled Message");
}
