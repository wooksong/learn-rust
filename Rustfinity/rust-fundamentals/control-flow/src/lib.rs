pub fn check_number_sign(number: i32) -> String {
    if number > 0 {
        return "positive".to_string();
    } else if number < 0 {
        return "negative".to_string();
    }

    "zero".to_string()
}
