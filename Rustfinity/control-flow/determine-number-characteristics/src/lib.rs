pub fn describe_number(n: i32) -> String {
    let mut desc = String::from(
        match n {
            n if n > 0 => "Positive",
            n if n < 0 => "Negative",
            _ => "Zero",
        }
    );
    
    if n != 0 {
        desc.push_str(
            if n % 2 == 0 { " even" } else { " odd" }
        );
    }   
    desc
}
