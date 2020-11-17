
pub fn fibonacci(index: u32) -> u32 {
    return if index <= 2 { 1 } else { fibonacci(index - 1) + fibonacci(index - 2) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(6), 8);
    }
}
