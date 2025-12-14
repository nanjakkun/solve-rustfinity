/*
Create an enum Animal with the following variants:

    Unit Struct Variant:
        Dog — represents a generic dog.

    Tuple Struct Variant:
        Cat(String) — represents a cat, where the String contains the cat's name.

    Named Field Struct Variant:
        Bird { species: String, can_fly: bool } — represents a bird with its species and whether it can fly.

Write a function describe_animal that takes a reference to an Animal and returns a String description based on the variant:

    For Dog, return "A friendly dog.".
    For Cat(name), return "A cat named {name}.".
    For Bird { species, can_fly }, return:
        "A {species} that can fly." if can_fly is true.
        "A {species} that cannot fly." if can_fly is false.
*/
pub enum Animal {
    // Define the Animal variants here
    Dog,
    Cat(String),
    Bird { species: String, can_fly: bool },
}

pub fn describe_animal(animal: &Animal) -> String {
    // Your code here...
    match animal {
        Animal::Dog => "A friendly dog.".to_string(),
        Animal::Cat(name) => format!("A cat named {}.", name),
        Animal::Bird { species, can_fly } => {
            if *can_fly {
                format!("A {species} that can fly.")
            } else {
                format!("A {species} that cannot fly.")
            }
        }
    }
}

// Example use case
pub fn main() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}
