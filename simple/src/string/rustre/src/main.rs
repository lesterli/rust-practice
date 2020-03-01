fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", uppercase(&arg));
    println!("arg = {}", arg);
}

fn uppercase(s: &str) -> String {
    s.to_uppercase()
}