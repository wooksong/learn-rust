use std::cmp::PartialOrd;
use std::fmt::Display;

pub fn compare_and_display<T>(lhs: T, rhs: T) -> T 
where
     T: Display + PartialOrd,
{
     if lhs > rhs {
          return lhs;
     }
     rhs
}

// Example usage
pub fn main() {
     let greater = compare_and_display(10, 20);
     println!("Greater value: {}", greater);

     let greater = compare_and_display("Apple", "Orange");
     println!("Greater value: {}", greater);
}
