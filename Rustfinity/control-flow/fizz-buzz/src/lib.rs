pub fn fizz_buzz(num: u32) -> String {
    match num {
        num if num % 15 == 0 => "FizzBuzz".to_string(), 
        num if num % 3 == 0 => "Fizz".to_string(),
        num if num % 5 == 0 => "Buzz".to_string(),
        _ => num.to_string(),
    }
}
