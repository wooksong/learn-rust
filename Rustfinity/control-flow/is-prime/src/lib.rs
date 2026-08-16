pub fn is_prime(n: u32) -> bool {
    let n = n as usize;
    if n < 2 {
        return false;
    }
    
    let mut sieve: Vec<bool> = vec![true; n + 1];
    
    sieve[0] = false;
    sieve[1] = false;
    
    for i in 2..n{        
        for j in (i+1)..=n {
            let is_p = &mut sieve[j];
            if !*is_p {
                continue;
            }
            if j % i == 0 {
                *is_p = false;
                continue;
            }
        }
    }
    sieve[n]
}
