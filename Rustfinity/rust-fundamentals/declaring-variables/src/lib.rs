pub fn calculate_area() -> u32 {
    let width = 30;
    let height = 20;

    prints_values(width, height);
    width * height
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}
