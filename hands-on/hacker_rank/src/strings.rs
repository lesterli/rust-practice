fn super_reduced_string(s: &str) -> String {
    let mut stack = Vec::new();

    for c in s.chars() {
        if stack.last() == Some(&c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    if stack.is_empty() {
        "Empty String".to_string()
    } else {
        stack.into_iter().collect()
    }
}

fn camelcase(s: &str) -> i32 {
    let mut count = 1;
    for c in s.chars() {
        if c.is_uppercase() {
            count += 1;
        }
    }
    return count;
}

fn alternate(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut max_length = 0;
    for c1 in 'a'..='z' {
        for c2 in (c1 as u8 + 1)..=b'z' {
            let a = c1 as char;
            let b = c2 as char;
            let mut last_char = None;
            let mut length = 0;
            for &c in &chars {
                if c != a && c != b {
                    continue;
                }
                if last_char == Some(c) {
                    length = 0;
                    break;
                }
                last_char = Some(c);
                length += 1;
            }
            if length > max_length {
                max_length = length;
            }
        }
    }
    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_reduced_string() {
        assert_eq!(super_reduced_string("aab"), "b");
        assert_eq!(super_reduced_string("abba"), "Empty String");
        assert_eq!(super_reduced_string("aaabccddd"), "abd");
        assert_eq!(super_reduced_string("abcdeedcba"), "Empty String");
        assert_eq!(super_reduced_string(""), "Empty String");
    }

    #[test]
    fn test_camelcase() {
        assert_eq!(camelcase("oneTwoThree"), 3);
        assert_eq!(camelcase("helloWorld"), 2);
        assert_eq!(camelcase("thisIsACamelCaseString"), 6);
    }

    #[test]
    fn test_alternate() {
        assert_eq!(alternate("abaacdabd"), 4);
        assert_eq!(alternate("beabeefeab"), 5);
    }
}
