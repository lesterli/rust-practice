//! Module 5 exercises: Error handling utilities + unit tests.

use std::fs;
use std::io;

/// Parse an integer from a string (demonstrates Result from std).
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse::<i32>()
}

/// Divide two integers, returning an error on division by zero.
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Read the entire file contents and return as String. Demonstrates `?` propagation.
pub fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // Using read_to_string which returns Result<String, io::Error>
    let contents = fs::read_to_string(path)?;
    Ok(contents.trim().to_string())
}

/// Example function that intentionally panics on non-positive input.
/// Used to demonstrate `#[should_panic]` in tests.
pub fn must_be_positive(n: i32) -> i32 {
    if n <= 0 {
        panic!("value must be positive, got {}", n);
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_parse_number_ok() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert_eq!(parse_number("  -7 \n").unwrap(), -7);
    }

    #[test]
    fn test_parse_number_err() {
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2).unwrap(), 5);
        assert!(safe_divide(1, 0).is_err());
    }

    #[test]
    fn test_read_username_from_file() {
        // create a temporary file in the OS temp dir
        let mut path: PathBuf = std::env::temp_dir();
        path.push("rust_test_username.txt");
        let path_str = path.to_str().unwrap();

        // write and read
        fs::write(&path, "alice\n").expect("write tmp file");
        let username = read_username_from_file(path_str).expect("read username");
        assert_eq!(username, "alice");

        // cleanup (ignore errors)
        let _ = fs::remove_file(path);
    }

    #[test]
    #[should_panic(expected = "value must be positive")]
    fn test_must_be_positive_panics() {
        // should panic
        let _ = must_be_positive(0);
    }
}