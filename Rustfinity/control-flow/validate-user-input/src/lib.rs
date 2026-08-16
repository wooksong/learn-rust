pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    if (age > 120) || (age < 0) {
        return Err("Invalid age".to_string());
    }
    if ! email.contains("@") {
        return Err("Invalid email".to_string());
    }
    Ok(())
}
