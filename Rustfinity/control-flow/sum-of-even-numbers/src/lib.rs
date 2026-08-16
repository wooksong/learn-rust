pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    for n in start..=end {
        if n % 2 == 0 {
            sum += n;
        }
    }
    sum
}
