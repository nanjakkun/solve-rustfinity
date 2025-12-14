pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    // Your code here
    let animal = Animal {
        name: name.to_string(),
        age,
    };
    Box::new(animal)
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    ((*animal.name).to_string(), animal.age)
}

// Example usage
pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}
