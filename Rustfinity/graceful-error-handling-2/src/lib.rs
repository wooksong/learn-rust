use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Input cannot be parsed as a number"),
            ParsePercentageError::OutOfRange => write!(f, "Number is not in the range 0-100"),
        }
    }
}

impl std::error::Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(num) => {
            if num > 100 {
                return Err(ParsePercentageError::OutOfRange);
            } else {
                return Ok(num);
            }
        },
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
