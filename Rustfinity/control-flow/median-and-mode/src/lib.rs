use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f32 / 2.0
    } else {
        numbers[len / 2] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut counts_map = HashMap::<i32, i32>::new();
    
    for &num in numbers {
        counts_map.entry(num).and_modify(|num| *num += 1).or_insert(1);
    }
    let mut ret: Vec<i32> = vec![];
    let max_count = match counts_map.values().max() {
        Some(&m) => m,
        None => return ret,
    };

    for (&k, &v) in &counts_map {
        if v != max_count {
            continue;
        }
        ret.push(k);
    }
    ret.sort_unstable();
    ret
}

