pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };
    for i in start..=end {
        let num = i.to_string();
        let reverse: String = i.to_string().chars().rev().collect();
        if num == reverse {
            return Some(i)
        }
    }
    None
}
