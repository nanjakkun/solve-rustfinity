/*
You need to define the StudentGrades struct, which contains a HashMap of student names (String) as keys and Student structs as values. Each Student struct should have the following fields:
    name: The name of the student (String).
    grades: A Vec<u8> to store the student's grades.

Implement the following methods for the StudentGrades struct:
    add_student(name: &str): Add a new student to the HashMap. If the student already exists, do nothing.
    add_grade(name: &str, grade: u8): Add a grade to an existing student.
    get_grades(name: &str) -> &[u8]: Retrieve the grades of a student as an immutable reference.
    Since there’s no error handling, assume all inputs are valid, and keys will exist when accessed.
*/

use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        self.students.entry(name.to_string()).or_insert(Student {
            name: name.to_string(),
            grades: Vec::new(),
        });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.grades.push(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        &self.students.get(name).unwrap().grades
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
