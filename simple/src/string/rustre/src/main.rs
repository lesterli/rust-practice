fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");
    println!("{}", arg.to_uppercase());
}
