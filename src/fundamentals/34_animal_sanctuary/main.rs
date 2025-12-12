/*
add_animal_to_section: This function should add an animal to a section in the registry. If the section does not exist, it should be created. If the animal is already in the section, it should not be added again.

get_animals_in_section: This function should return a list of animals sorted alphabetically in a given section. If the section does not exist, it should return an empty list.

get_all_animals_sorted: This function should return a copy of the entire registry with all animals sorted alphabetically in each section.
*/

use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let section_animals = registry.entry(section.to_string()).or_insert(Vec::new());
    if !section_animals.contains(&animal.to_string()) {
        section_animals.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    match registry.get(section) {
        Some(animals) => {
            let mut sorted_animals = animals.clone();
            sorted_animals.sort();
            sorted_animals
        }
        None => Vec::new(),
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_animals: Vec<String> = registry
        .values()
        .flat_map(|animals| animals.iter())
        .cloned()
        .collect();
    all_animals.sort();
    all_animals
}

/////////////////////////////

pub fn main() {
    let mut registry = Collection::new();

    add_animal_to_section("Eagle", "Birds", &mut registry);
    assert_eq!(get_animals_in_section("Birds", &registry), vec!["Eagle"]);
}
