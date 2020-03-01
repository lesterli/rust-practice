fn main() {
    let arg = std::env::args_os()
        .skip(1)
        .next()
        .expect("should have one argument");

    match arg.to_str() {
        Some(arg) => println!("valid UTF-8: {}", arg),
        None => println!("not valid UTF-8: {:?}", arg),
    }
}
