pub fn filter_starts_with<'a>(
    given: &'a [String],
    keyword: &'a str,
) -> impl Iterator<Item = &'a String> {
    given.iter().filter(move |s| s.starts_with(keyword))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
