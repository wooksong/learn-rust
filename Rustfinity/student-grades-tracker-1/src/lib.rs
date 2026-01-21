use std::collections::HashMap;

pub struct Student {
    name: String,
    grades: Vec<u8>,
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
        let map = &mut self.students;
        let student = Student {
            name: String::from(name),
            grades: Vec::<u8>::new(),
        };

        map.insert(String::from(&student.name), student);
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        let map = &mut self.students;

        if let Some(student) = map.get_mut(&name.to_string()) {
            student.grades.push(grade);
        }
        
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        let map = &self.students;
        let student = map.get(&name.to_string());
        
        &student.unwrap().grades
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
