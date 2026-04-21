// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    pub text: &'a mut String,
}

// 2. Implement the methods for the struct
impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }

    pub fn find_first(&self, key: &str) -> Option<&str> {
        for line in self.text.lines() {
            if line.contains(key) {
                return Some(line);
            }
        }

        None
    }

    pub fn replace_lines(&mut self, source: &str, target: &str) {
        let lines: Vec<String> = self
            .text
            .lines()
            .map(|line| {
                if line.contains(source) {
                    target.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        *self.text = lines.join("\n");
    }

    pub fn get_text(&self) -> &str {
        self.text
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
