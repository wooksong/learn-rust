pub fn countdown(n: u32) -> Vec<u32> {
    let mut counter = n;
    let mut v: Vec<u32> = vec![];
    while counter != 0 {
        v.push(counter);
        counter -= 1;
    }
    v.push(0);
    v
}
