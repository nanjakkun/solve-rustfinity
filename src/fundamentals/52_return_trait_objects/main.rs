/*
    Define the Speakable trait with a method speak that returns a String.
    Define a struct Dog with two fields: name and breed, both of type String.
    Implement the Speakable trait for Dog to return a string Woof.
    Define a struct Robot with two fields: model and purpose, both of type String.
    Implement the Speakable trait for Robot to return a string Beep boop.
    Finish the function get_speaker that takes a &str parameter and returns either a Dog or a Robot based on the parameter.
    The parameter can either be dog or robot.
*/

pub struct Dog {
    pub name: String,
    pub breed: String,
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

pub trait Speakable {
    fn speak(&self) -> String;
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        "Beep boop".to_string()
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => Box::new(Dog {
            name: String::from("Pochi"),
            breed: String::from("Shiba"),
        }),
        "robot" => Box::new(Robot {
            model: String::from("Asimo"),
            purpose: String::from("Assistance"),
        }),
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
