fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for b in arg.bytes() {
        print!("{:02X} ", b);
    }
    println!()
}