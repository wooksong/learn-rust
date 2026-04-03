pub fn print_message(msg: impl AsRef<str> + std::fmt::Display) {
    println!("{msg}");
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
