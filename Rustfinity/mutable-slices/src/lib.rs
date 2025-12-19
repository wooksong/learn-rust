pub fn transform_even_odd(slice: &mut [i32]) {
    for elem in slice.iter_mut() {
        match *elem % 2{
            0 => *elem *= 2,
            _ => *elem -= 1,
        }
    }
}
