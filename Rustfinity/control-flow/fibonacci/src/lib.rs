pub fn fibonacci(n: u32) -> u32 {
    let (mut cur, mut next) = (0, 1);
    
    for _ in 0..n {
        (cur, next) = (next, cur + next);
    }
    cur    
}
