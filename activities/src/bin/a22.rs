// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
pub fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
pub fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
pub fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        let result = clamp(4, 3, 6);
        let expected = 4;
        assert_eq!(result, expected, "clamp not working");
    }

    #[test]
    fn test_clamp_lower() {
        let result = clamp(2, 3, 6);
        let expected = 3;
        assert_eq!(result, expected, "clamp not working");
    }

    #[test]
    fn test_clamp_upper() {
        let result = clamp(7, 3, 6);
        let expected = 6;
        assert_eq!(result, expected, "clamp not working");
    }

    #[test]
    fn test_divide() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "divide is not working");
    }

    #[test]
    fn test_divide_by_zero() {
        let result = div(10, 0);
        let expected = None;
        assert_eq!(result, expected, "divide is not working");
    }

    #[test]
    fn test_concat() {
        let result = concat("First", "Second");
        let expected = "First Second".to_owned();
        assert_eq!(result, expected, "concat is not working");
    }
}
