pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    numbers
        .first()
        .copied()
        .ok_or_else(|| String::from("Vector is empty"))
        .and_then(|val| {
            if val < min_value {
                Err(String::from(
                    "First element is below the minimum allowed value",
                ))
            } else {
                Ok(val)
            }
        })
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
