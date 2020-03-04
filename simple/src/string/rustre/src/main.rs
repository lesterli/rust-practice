fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    let mut upp = String::new();
    // was just `upp`
    uppercase(&arg, &mut upp);

    println!("upp = {}", upp);
    println!("arg = {}", arg);
}

// was `mut dst: String`
fn uppercase(src: &str, dst: &mut String) {
    for c in src.chars() {
        for c in c.to_uppercase() {
            dst.push(c);
        }
    }
}