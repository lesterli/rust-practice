fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for c in arg.chars() {
        print!("{} (U+{:04X}) ", c, c as u32);
    }
    println!()
}
