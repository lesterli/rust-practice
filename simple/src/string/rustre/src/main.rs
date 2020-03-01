fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for c in arg.chars() {
        print!("{} ", c);
    }
    println!()
}
