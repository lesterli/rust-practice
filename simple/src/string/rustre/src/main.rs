fn main() {
    let arg = std::env::args_os()
        .skip(1)
        .next()
        .expect("should have one argument");
    println!("{:?}", arg)
}
