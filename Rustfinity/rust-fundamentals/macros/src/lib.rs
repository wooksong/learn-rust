#[macro_export]
macro_rules! math_operations {
    ($lhs:expr, $ops:tt, $rhs:expr) => {
        match $ops {
            "+" => {
                format!(
                    "{} {} {} = {}",
                    $lhs,
                    $ops.trim_matches('"'),
                    $rhs,
                    $lhs + $rhs
                )
            }
            "-" => {
                format!(
                    "{} {} {} = {}",
                    $lhs,
                    $ops.trim_matches('"'),
                    $rhs,
                    $lhs - $rhs
                )
            }
            "*" => {
                format!(
                    "{} {} {} = {}",
                    $lhs,
                    $ops.trim_matches('"'),
                    $rhs,
                    $lhs * $rhs
                )
            }
            "/" => {
                if $rhs != 0 {
                    format!(
                        "{} {} {} = {}",
                        $lhs,
                        $ops.trim_matches('"'),
                        $rhs,
                        $lhs / $rhs
                    )
                } else {
                    panic!("Division by zero")
                }
            }
            _ => panic!("Unsupported operator"),
        }
    };
}

// Example usage
pub fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
}
