pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {0}, Age: {1}", self.name, self.age).to_string()
    }
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {0}, Author: {1}", self.title, self.author).to_string()
    }
}

pub fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };

    println!("{}", person.describe());
    println!("{}", book.describe());
}
