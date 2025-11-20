// Slice examples: &[T] and &str
pub fn run() {
    println!("-- slices module --");
    slices_example();
    string_slice_demo();
}

fn slices_example() {
    let mut foo = [0u8; 5];
    foo[1] = 1;
    foo[2] = 2;

    let bar = &foo[..3];
    println!("array: {:?}  slice (..3): {:?}", foo, bar);
}

fn string_slice_demo() {
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("first_word of '{}' is '{}'", s, first);

    // show safe slicing that won't panic
    let full = safe_split_first(&s);
    println!("safe_split_first -> '{}'", full);
}

fn first_word(s: &str) -> &str {
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn safe_split_first(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}