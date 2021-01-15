pub fn fibonacci(index: u32) -> u32 {
    if index <= 2 {
        1
    } else {
        fibonacci(index - 1) + fibonacci(index - 2)
    }
}

pub fn count_char(s: &str) -> u32 {
    s.chars().count() as u32
}

pub fn handle_tuple(pair: (u32, bool)) -> (u32, bool) {
    let (integer, boolean) = pair;

    (integer + 1, !boolean)
}

pub fn sum_of_even_fixed(array: [i32; 10]) -> i32 {
    array.iter()
    .filter(|&&num| num % 2 == 0)
    .fold(0, |sum, &num| sum + num)
}

pub fn sum_of_even(slice: &[i32]) -> i32 {
    slice.iter()
    .filter(|&&num| num % 2 == 0)
    .fold(0, |sum, &num| sum + num)
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

    #[test]
    fn test_count_char() {
        let hello = "hello";
        assert_eq!(count_char(hello), 5);
        let hello_utf8 = "你好";
        assert_eq!(count_char(hello_utf8), 2);
    }

    #[test]
    fn test_handle_tuple() {
        let pair = (100, true);
        assert_eq!(handle_tuple(pair), (101, false));
    }

    #[test]
    fn test_sum_of_even() {
        let array = [1,2,3,4,5,6,7,8,9,10];
        assert_eq!(sum_of_even(&array), 30);
        assert_eq!(sum_of_even(&array[..5]), 6);
    }
}
