pub fn factorial(n: u32) -> u128 {
    let mut f: u128 = 1;
    for num in 1..=n.into() {
        f *= num;
    }
    f
}
