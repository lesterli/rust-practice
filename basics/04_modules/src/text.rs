// String examples
pub fn run() {
    println!("-- text (String) module --");
    string_example();
}

fn string_example() {
    let mut s = String::from("str");
    s.push_str("add");
    println!("s {}", s);

    // iterate characters and bytes
    println!("chars:");
    for c in s.chars() {
        println!("  char {}", c);
    }

    println!("bytes:");
    for b in s.bytes() {
        println!("  byte {}", b);
    }
}