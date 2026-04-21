pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for i in indices.iter() {
        let v = slice.get_mut(*i);

        if v == None {
            continue;
        }
        slice[*i] = value;
    }
}
