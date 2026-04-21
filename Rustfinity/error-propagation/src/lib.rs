use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let file = File::open(file_path)?;
    let br = BufReader::new(file);

    let mut sum = 0;

    for line in br.lines() {
        let num = line?
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        sum += num;
    }

    Ok(sum)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
