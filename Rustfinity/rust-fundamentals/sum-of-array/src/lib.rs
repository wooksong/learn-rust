pub fn sum_array(arr: &[i32]) -> i32 {
    IntoIterator::into_iter(arr).sum()
}
