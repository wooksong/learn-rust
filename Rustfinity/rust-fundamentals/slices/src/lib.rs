pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }

    let mut max_n: i32 = slice[0];

    for i in slice[1..].iter() {
        if max_n < *i {
            max_n = *i
        }
    }

    Some(max_n)
}

// Example Usage
pub fn main() {
    let numbers = [1, 3, 7, 2, 5];
    assert_eq!(find_largest_in_slice(&numbers), Some(7));

    let empty: [i32; 0] = [];
    assert_eq!(find_largest_in_slice(&empty), None);

    let single_element = [42];
    assert_eq!(find_largest_in_slice(&single_element), Some(42));
}
