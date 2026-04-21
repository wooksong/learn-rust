#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

// Example function
pub fn debug_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let point = Point { x: 5.0, y: -3.2 };
    println!("{:?}", point);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rectangle);
}
